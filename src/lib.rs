//! Barebones baseview vst2 plugin that logs events to ~/tmp/BaseviewTest.log

#[macro_use]
extern crate vst;

use baseview::{
    Size, Event, Window, WindowHandler, WindowOpenOptions,
    WindowScalePolicy
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vst::plugin::{Info, Plugin};
use vst::editor::Editor;


const PLUGIN_NAME: &str = "BaseviewTest";

const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 500;


#[derive(Default)]
struct TestWindowHandler;


impl WindowHandler for TestWindowHandler {
    fn on_event(&mut self, _: &mut Window, event: Event) {
        ::log::info!("TestWindowHandler received event: {:?}", event)
    }

    fn on_frame(&mut self, _window: &mut Window) {
        
    }
}


#[derive(Default)]
struct TestPluginEditor {
    is_open: bool,
}


impl Editor for TestPluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        if self.is_open {
            return false;
        }

        self.is_open = true;

        let options = WindowOpenOptions {
            title: PLUGIN_NAME.to_string(),
            size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_parented(&VstParent(parent), options, |_|{
            TestWindowHandler::default()
        });

        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}


struct TestPlugin {
    editor: Option<TestPluginEditor>,
}


impl Default for TestPlugin {
    fn default() -> Self {
        Self {
            editor: Some(TestPluginEditor::default()),
        }
    }
}


impl Plugin for TestPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            unique_id: 53435,

            ..Default::default()
        }
    }

    fn init(&mut self) {
        init_logging();
    }   

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}


fn init_logging(){
    let log_folder = ::dirs::home_dir().unwrap().join("tmp");

    let _ = ::std::fs::create_dir(log_folder.clone());

    let log_file = ::std::fs::File::create(
        log_folder.join(format!("{}.log", PLUGIN_NAME))
    ).unwrap();

    let log_config = ::simplelog::ConfigBuilder::new()
        .set_time_to_local(true)
        .build();

    let _ = ::simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        log_config,
        log_file
    );  

    ::log_panics::init();

    ::log::info!("init");
}


struct VstParent(*mut ::std::ffi::c_void);


#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0 as *mut ::std::ffi::c_void,
            ..MacOSHandle::empty()
        })
    }
}


#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}


#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}


plugin_main!(TestPlugin);
