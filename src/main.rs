extern crate gtk; // ����������� ���������� GTK
use gtk::prelude::*; // ����������� �������� GTK, ����������� ��� ����������� �������� � ������������
use gtk::{Label, Button, Entry, ScrolledWindow, TextBuffer, TextView, Window, WindowType}; // ����������� ����������� ������� GTK

fn main() {
    // ������������� GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // �������� ����
    let window = Window::new(WindowType::Toplevel); // ������� ����� ���� �������� ������
    window.set_title("Notes"); // ������������� ��������� ����
    window.set_default_size(400, 300); // ������������� ������� ����

    // �������� ��������
    let label = Label::new(Some("Add notes:")); // ������� ����� � �������
    let entry = Entry::new(); // ������� ���� ��� ����� ������
    let button = Button::new_with_label("Add"); // ������� ������ � ��������� ������
    let scrolled_window = ScrolledWindow::new(None::<&TextBuffer>, None::<&TextView>); // ������� ���� �� �����������
    let text_view = TextView::new(); // ������� ������� ��� ����������� ������

    // ����������� ������ ������
    let buffer = text_view.get_buffer().unwrap(); // �������� ����� ������ ��� ���������� ������� � �������

    // ���������� �������� � ���������
    scrolled_window.add(&text_view); // ��������� ������� ��� ������ � ���� �� �����������

    // �������� ������������� ����������
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5); // ������� ������������ ��������� � ����������� ����� ���������
    vbox.pack_start(&label, false, false, 0); // ��������� ����� � ���������
    vbox.pack_start(&entry, false, false, 0); // ��������� ���� ����� � ���������
    vbox.pack_start(&button, false, false, 0); // ��������� ������ � ���������
    vbox.pack_start(&scrolled_window, true, true, 0); // ��������� ���� �� ����������� � ���������

    // ���������� ������������� ���������� � ����
    window.add(&vbox); // ��������� ������������ ��������� � ���� �������� ������

    // ���������� ������� ����� �� ������
    button.connect_clicked(move |_| { // ������������� ���������� ����� �� ������
        let text = entry.get_text().unwrap().to_string(); // �������� ����� �� ���� �����
        buffer.insert_at_cursor(&text); // ��������� ����� � ������� ������
        buffer.insert_at_cursor("\n"); // ��������� ������� ������
        entry.set_text(""); // ������� ���� �����
    });

    // ��������� ������� �������� ����
    window.connect_delete_event(|_, _| {
        // ���������� ���������� ��� �������� ����
        gtk::main_quit();
        Inhibit(false)
    });

    // ����������� ���� ���������
    window.show_all(); // ���������� ��� ������� � ����

    // ������ ��������� ����� GTK
    gtk::main(); // ��������� �������� ���� GTK ��� ��������� �������
}

