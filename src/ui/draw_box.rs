use crate::screen::{TextBufferRect, TextBufferInterface, TextAlign};

pub fn draw_box(rect: TextBufferRect, screen: &mut impl TextBufferInterface, name: Option<(&str, TextAlign)>) {
    assert!(rect.width >= 8);

    let top_s = if let Some((name, align)) = name {
        let name = if name.len() + 4 <= rect.width {
            name.to_string()
        }
        else {
            format!("{}...", &name[..rect.width - 7])
        };

        let left = (rect.width - 4 - name.len() + 1) / 2;
        let right = (rect.width - 4 - name.len()) / 2;

        let mut s = String::from("┌");

        if align == TextAlign::Left {
            s += "┤";
            s += &name;
            s += "├";
        }

        for _ in 0..left {
            s += "─";
        }

        if align == TextAlign::Center {
            s += "┤";
            s += &name;
            s += "├";
        }

        for _ in 0..right {
            s += "─";
        }

        if align == TextAlign::Right {
            s += "┤";
            s += &name;
            s += "├";
        }

        s += "┐";

        s
    }
    else {
        let mut s = String::from("┌");

        for _ in 1..rect.width - 1 {
            s += "─";
        }

        s += "┐";

        s
    };
    let bottom_s = {
        let mut s = String::from("└");

        for _ in 1..rect.width - 1 {
            s += "─";
        }

        s += "┘";

        s
    };

    screen.write_string((rect.x, rect.y).into(), &top_s, crate::screen::CharacterColor::BrightWhite).unwrap();
    screen.write_string((rect.x, rect.bottom() - 1).into(), &bottom_s, crate::screen::CharacterColor::BrightWhite).unwrap();

    for i in rect.y + 1..rect.bottom() - 1 {
        screen.write_string((rect.x, i).into(), "│", crate::screen::CharacterColor::BrightWhite).unwrap();
        screen.write_string((rect.right() - 1, i).into(), "│", crate::screen::CharacterColor::BrightWhite).unwrap();
    }
}