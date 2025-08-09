use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, gdk};

// 定义主窗口结构体，用于聚合相关组件
pub struct MainWindow {
    pub window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = create_main_window(app);
        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

pub fn create_main_window(app: &Application) -> ApplicationWindow {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(800)
        .default_height(600)
        .build();

    // 设置窗口在屏幕中央
    window.set_resizable(true);

    // Create main layout
    let vbox = create_main_layout();

    // Add CSS styling
    add_css_styling();

    // Present window
    window.set_child(Some(&vbox));

    window
}

fn create_main_layout() -> gtk::Box {
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Create a grid for sidebar and main content
    let grid = gtk::Grid::builder()
        .column_spacing(0)
        .row_spacing(0)
        .build();

    // Create sidebar
    let sidebar = create_sidebar();

    // Create main content area
    let main_content = create_main_content();

    // Create a draggable separator
    let separator = create_draggable_separator(&sidebar);

    // Attach widgets to grid using grid layout
    // Sidebar in column 0
    grid.attach(&sidebar, 0, 0, 1, 1);
    // Separator in column 1
    grid.attach(&separator, 1, 0, 1, 1);
    // Main content in column 2
    grid.attach(&main_content, 2, 0, 1, 1);

    // Set column properties
    grid.set_column_homogeneous(false);
    // Sidebar can expand
    sidebar.set_hexpand(true);
    // Main content expands to fill available space
    main_content.set_hexpand(true);
    // Both sidebar and main content expand vertically
    sidebar.set_vexpand(true);
    main_content.set_vexpand(true);

    // Add the grid to the vbox
    vbox.append(&grid);

    vbox
}

fn create_sidebar() -> gtk::Box {
    let sidebar = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .hexpand(true)
        .vexpand(true)
        .build();

    // Add CSS class for sidebar border
    sidebar.add_css_class("sidebar");

    // Add some sample sidebar content
    let sidebar_label = gtk::Label::builder()
        .label("Sidebar")
        .css_classes(["heading"])
        .build();

    let sidebar_button1 = gtk::Button::builder().label("Item 1").build();

    let sidebar_button2 = gtk::Button::builder().label("Item 2").build();

    let sidebar_button3 = gtk::Button::builder().label("Item 3").build();

    // 创建打开工具窗口的按钮
    let tools_button = gtk::Button::builder().label("Open Tools").build();

    tools_button.connect_clicked(|_| {
        // 创建并显示工具窗口
        let tools_window = super::tools_window::ToolsWindow::new();
        tools_window.present();
    });

    sidebar.append(&sidebar_label);
    sidebar.append(&sidebar_button1);
    sidebar.append(&sidebar_button2);
    sidebar.append(&sidebar_button3);
    sidebar.append(&tools_button);

    sidebar
}

fn create_main_content() -> gtk::Box {
    let main_content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .hexpand(true)
        .vexpand(true)
        .build();

    // Add some sample main content
    let content_label = gtk::Label::builder()
        .label("Main Content Area")
        .css_classes(["heading"])
        .build();

    let content_text = gtk::Label::builder()
        .label("This is the main content area. Your application content goes here.")
        .wrap(true)
        .build();

    main_content.append(&content_label);
    main_content.append(&content_text);

    main_content
}

fn create_draggable_separator(sidebar: &gtk::Box) -> gtk::Separator {
    let separator = gtk::Separator::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["sidebar-separator"])
        .build();

    // Create drag gesture for the separator
    let drag_gesture = gtk::GestureDrag::new();
    drag_gesture.set_button(1); // Left mouse button

    // Make separator draggable
    drag_gesture.connect_drag_begin(move |gesture, _, _| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
    });

    let sidebar_clone = sidebar.clone();
    drag_gesture.connect_drag_update(move |gesture, offset_x, _| {
        // 使用起始位置和偏移量计算新宽度，避免累积误差
        if let Some(start_x) = gesture.start_point().map(|p| p.0) {
            let new_width = (start_x + offset_x) as i32;
            // 确保宽度不小于-1，避免GTK断言失败
            let final_width = if new_width < -1 { -1 } else { new_width };
            sidebar_clone.set_size_request(final_width, -1);
        }
    });

    // 添加drag_end事件处理，确保拖拽结束后正确设置宽度
    let sidebar_clone = sidebar.clone();
    drag_gesture.connect_drag_end(move |gesture, _, _| {
        // 使用起始位置和偏移量计算最终宽度
        if let (Some(start_x), Some((offset_x, _))) = (gesture.start_point().map(|p| p.0), gesture.offset()) {
            let new_width = (start_x + offset_x) as i32;
            // 确保宽度不小于-1
            let final_width = if new_width < -1 { -1 } else { new_width };
            sidebar_clone.set_size_request(final_width, -1);
        }
    });

    // Add drag gesture to separator
    separator.add_controller(drag_gesture);

    // 增强分割线的鼠标悬停效果
    let motion_controller = gtk::EventControllerMotion::new();
    let separator_clone = separator.clone();
    motion_controller.connect_enter(move |_, _, _| {
        separator_clone.add_css_class("sidebar-separator-hover");
    });

    let separator_clone = separator.clone();
    motion_controller.connect_leave(move |_| {
        separator_clone.remove_css_class("sidebar-separator-hover");
    });

    separator.add_controller(motion_controller);

    separator
}

fn add_css_styling() {
    // Add CSS provider for sidebar styling
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        "
        .sidebar {
            border: 1px solid #ccc;
            border-radius: 0px;
            padding: 0px;
            background: #f6f6f6;
            margin-left: 0px;
            margin-top: 0px;
            margin-bottom: 0px;
            margin-right: 0px;
            min-width: 120px;
        }
        box {
            margin: 0px;
            padding: 0px;
        }
        window {
            margin: 0;
            padding: 0;
        }
        .content-area {
        }
        .sidebar-separator {
            background-color: #ccc;
            min-width: 2px;
            cursor: ew-resize;  /* 修改为左右方向箭头 */
            /* 确保分隔器可以接收鼠标事件 */
            -gtk-icon-source: none;
        }
        .sidebar-separator-hover {
            background-color: #999;
            min-width: 4px;
            transition: all 0.2s ease;
        }
    ",
    );
    let display = gdk::Display::default().unwrap();
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}