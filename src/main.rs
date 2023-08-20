extern crate gtk; // Импортируем библиотеку GTK
use gtk::prelude::*; // Импортируем прелюдию GTK, необходимую для подключения сигналов и обработчиков
use gtk::{Label, Button, Entry, ScrolledWindow, TextBuffer, TextView, Window, WindowType}; // Импортируем необходимые виджеты GTK

fn main() {
    // Инициализация GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Создание окна
    let window = Window::new(WindowType::Toplevel); // Создаем новое окно верхнего уровня
    window.set_title("Notes"); // Устанавливаем заголовок окна
    window.set_default_size(400, 300); // Устанавливаем размеры окна

    // Создание виджетов
    let label = Label::new(Some("Add notes:")); // Создаем метку с текстом
    let entry = Entry::new(); // Создаем поле для ввода текста
    let button = Button::new_with_label("Add"); // Создаем кнопку с текстовой меткой
    let scrolled_window = ScrolledWindow::new(None::<&TextBuffer>, None::<&TextView>); // Создаем окно со скроллингом
    let text_view = TextView::new(); // Создаем область для отображения текста

    // Определение буфера текста
    let buffer = text_view.get_buffer().unwrap(); // Получаем буфер текста для управления текстом в области

    // Добавление виджетов в контейнер
    scrolled_window.add(&text_view); // Добавляем область для текста в окно со скроллингом

    // Создание вертикального контейнера
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5); // Создаем вертикальный контейнер с расстоянием между виджетами
    vbox.pack_start(&label, false, false, 0); // Добавляем метку в контейнер
    vbox.pack_start(&entry, false, false, 0); // Добавляем поле ввода в контейнер
    vbox.pack_start(&button, false, false, 0); // Добавляем кнопку в контейнер
    vbox.pack_start(&scrolled_window, true, true, 0); // Добавляем окно со скроллингом в контейнер

    // Добавление вертикального контейнера в окно
    window.add(&vbox); // Добавляем вертикальный контейнер в окно верхнего уровня

    // Обработчик события клика на кнопке
    button.connect_clicked(move |_| { // Устанавливаем обработчик клика на кнопке
        let text = entry.get_text().unwrap().to_string(); // Получаем текст из поля ввода
        buffer.insert_at_cursor(&text); // Вставляем текст в область текста
        buffer.insert_at_cursor("\n"); // Добавляем перенос строки
        entry.set_text(""); // Очищаем поле ввода
    });

    // Обработка события закрытия окна
    window.connect_delete_event(|_, _| {
        // Завершение приложения при закрытии окна
        gtk::main_quit();
        Inhibit(false)
    });

    // Отображение всех элементов
    window.show_all(); // Отображаем все виджеты в окне

    // Запуск основного цикла GTK
    gtk::main(); // Запускаем основной цикл GTK для обработки событий
}

