use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;

fn main() {
	// Create a new GTK app instance
    let app = Application::builder()
    	.application_id("com.jwestall.gtk_ex")
    	.build();
    
    // Build the ui on app activate
    app.connect_activate(build_ui);
    
    app.run();
}

fn build_ui(app: &Application) {
	// Create a button widget
	let button = Button::builder()
		.label("Press me!")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();
	
	// Change label on button click
    button.connect_clicked(move |button| {
    	button.set_label("Hello World!");
    });

	// Create a new application window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("GTK App")
		.child(&button)
		.build();
	
	window.present();
}
