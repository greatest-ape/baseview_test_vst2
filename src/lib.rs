/// Barebones baseview vst2 plugin that logs events to ~/tmp/BaseviewTest.log

#[macro_use]
extern crate vst;

use baseview::{
    Size, Event, Parent, Window, WindowHandle, WindowHandler,
    WindowOpenOptions, WindowScalePolicy
};
use raw_window_handle::RawWindowHandle;
use vst::plugin::{Info, Plugin};
use vst::editor::Editor;


const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 500;


struct TestWindowHandler;


impl Default for TestWindowHandler {
    fn default() -> Self {
        Self
    }
}


impl WindowHandler for TestWindowHandler {
    type Message = ();

    fn on_message(&mut self, _: &mut Window, message: Self::Message) {
        ::log::info!("TestWindowHandler received message: {:?}", message)
    }

    fn on_event(&mut self, _: &mut Window, event: Event) {
        ::log::info!("TestWindowHandler received event: {:?}", event)
    }

    fn on_frame(&mut self) {
        
    }
}


#[derive(Default)]
struct TestPluginEditor {
    handle: Option<WindowHandle>,
}


impl Editor for TestPluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        let parent = raw_window_handle_from_parent(parent);

        let options = WindowOpenOptions {
            title: "Baseview Test".to_string(),
            size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
            scale: WindowScalePolicy::ScaleFactor(1.0),
            parent: Parent::WithParent(parent),
        };

        self.handle = Some(Window::open(options, |_|{
            TestWindowHandler::default()
        }));

        true
    }

    fn is_open(&mut self) -> bool {
        self.handle.is_some()
    }

    fn close(&mut self) {
        self.handle = None;
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
            name: "Basic Plugin".to_string(),
            unique_id: 53435,

            ..Default::default()
        }
    }

    fn init(&mut self) {
        let log_folder = ::dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

        let log_file = ::std::fs::File::create(
            log_folder.join("BaseviewTest.log")
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


    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}


#[cfg(target_os = "macos")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::macos::MacOSHandle;
    use cocoa::base::id;
    use objc::{msg_send, sel, sel_impl};

    let ns_view = parent as id;

    let ns_window: id = unsafe {
        msg_send![ns_view, window]
    };

    RawWindowHandle::MacOS(MacOSHandle {
        ns_window: ns_window as *mut ::std::ffi::c_void,
        ns_view: ns_view as *mut ::std::ffi::c_void,
        ..MacOSHandle::empty()
    })
}


plugin_main!(TestPlugin);