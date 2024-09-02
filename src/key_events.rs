use crate::{app::App, todo::Todo};

fn key_char_q(app: &mut App) {
    app.is_running = false
}

fn key_char_n(app: &mut App) {
    app.todos.push(Todo::default());
    app.current_todo = app.todos.len() - 1;
    app.editing = true
}

fn key_char_d(app: &mut App) {
    if app.todos.len() > 1 {
        app.todos.remove(app.current_todo);
        if app.current_todo > 0 {
            app.current_todo -= 1;
        }
    } else {
        app.todos.push(Todo::default());
        app.todos.remove(0);
    }
}

fn key_char_space(app: &mut App) {
    app.todos[app.current_todo].completed = !app.todos[app.current_todo].completed;
}

pub fn key_down(app: &mut App) {
    if app.current_todo == app.todos.len() - 1 {
        app.current_todo = 0;
    } else {
        app.current_todo += 1;
    }
}

pub fn key_up(app: &mut App) {
    if app.current_todo == 0 {
        app.current_todo = app.todos.len() - 1;
    } else {
        app.current_todo -= 1;
    }
}

pub fn key_enter(app: &mut App) {
    app.editing = !app.editing;
}

pub fn key_esc(app: &mut App) {
    app.editing = false;
}

pub fn key_backspace(app: &mut App) {
    if app.editing {
        let mut chars = app.todos[app.current_todo].text.chars();
        chars.next_back();
        app.todos[app.current_todo].text = chars.as_str().to_string();
    }
}

pub fn key_char(app: &mut App, c: char) {
    if app.editing {
        app.todos[app.current_todo].text += &c.to_string();
    } else {
        match c {
            'q' => key_char_q(app),
            'n' => key_char_n(app),
            'd' => key_char_d(app),
            ' ' => key_char_space(app),
            _ => {}
        }
    }
}
