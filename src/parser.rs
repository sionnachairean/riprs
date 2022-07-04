use crate::structs::*;
use nom::{
    bytes::streaming::take_while_m_n,
    character::{
        complete::{char, one_of},
        is_digit,
    },
    combinator::{fail, map_res},
    multi::{fill, many1},
    sequence::{preceded, tuple},
    IResult,
};

const COMMAND_CHARS: &str = " !\"#$%&'()*+,-./:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn from_bool(input: &str) -> Result<bool, std::num::ParseIntError> {
    u32::from_str_radix(input, 2).and_then(|n| Ok(n != 0))
}

fn is_bool_digit(c: char) -> bool {
    c.is_digit(2)
}

fn bool(input: &str) -> IResult<&str, bool> {
    map_res(take_while_m_n(1, 1, is_bool_digit), from_bool)(input)
}

fn from_meganum(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 36)
}

fn is_meganum_digit(c: char) -> bool {
    c.is_digit(36)
}

fn meganum2(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(2, 2, is_meganum_digit), from_meganum)(input)
}

fn font_size(input: &str) -> IResult<&str, FontSize> {
    let (rest, size_char) = one_of("01234")(input)?;
    if let Ok(size) = FontSize::try_from(size_char.to_digit(36).unwrap()) {
        Ok((rest, size))
    } else {
        fail(input)
    }
}

fn palette_color(input: &str) -> IResult<&str, PaletteColor> {
    let (rest, color_index) = meganum2(input)?;
    if let Ok(color) = PaletteColor::try_from(color_index as u8) {
        Ok((rest, color))
    } else {
        fail(input)
    }
}

fn ega_color(input: &str) -> IResult<&str, EGAColor> {
    let (rest, color_index) = meganum2(input)?;
    if let Ok(color) = EGAColor::try_from(color_index as u8) {
        Ok((rest, color))
    } else {
        fail(input)
    }
}

fn write_mode(input: &str) -> IResult<&str, WriteMode> {
    let (rest, mode_index) = meganum2(input)?;
    if let Ok(mode) = WriteMode::try_from(mode_index) {
        Ok((rest, mode))
    } else {
        fail(input)
    }
}

pub fn ripscrip(input: &str) -> IResult<&str, Vec<Command>> {
    preceded(char('!'), many1(command))(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    preceded(char('|'), command_inner)(input)
}

fn command_inner(input: &str) -> IResult<&str, Command> {
    let (rest, (level, symbol)) =
        tuple((command_level, one_of(COMMAND_CHARS)))(input)?;

    let f = match (level, symbol) {
        ("", 'w') => rip_text_window,
        ("", 'v') => rip_viewport,
        ("", '*') => rip_reset_windows,
        ("", 'e') => rip_erase_window,
        ("", 'E') => rip_erase_view,
        ("", 'g') => rip_gotoxy,
        ("", 'H') => rip_home,
        ("", '>') => rip_erase_eol,
        ("", 'c') => rip_color,
        ("", 'Q') => rip_set_palette,
        ("", 'a') => rip_one_palette,
        ("", 'W') => rip_write_mode,
        _ => unknown,
    };
    f(rest)
}

fn command_level(input: &str) -> IResult<&str, &str> {
    take_while_m_n(0, 9, |c| is_digit(c as u8))(input)
}

fn unknown(_input: &str) -> IResult<&str, Command> {
    Ok(("", Command::Unknown))
}

fn rip_text_window(input: &str) -> IResult<&str, Command> {
    let (rest, (x0, y0, x1, y1, wrap, size)) = tuple((
        meganum2, meganum2, meganum2, meganum2, bool, font_size,
    ))(input)?;
    Ok((
        rest,
        Command::TextWindow {
            corners: (XY { x: x0, y: y0 }, XY { x: x1, y: y1 }),
            wrap,
            size,
        },
    ))
}

fn rip_viewport(input: &str) -> IResult<&str, Command> {
    let (rest, (x0, y0, x1, y1)) =
        tuple((meganum2, meganum2, meganum2, meganum2))(input)?;
    Ok((
        rest,
        Command::Viewport {
            corners: (XY { x: x0, y: y0 }, XY { x: x1, y: y1 }),
        },
    ))
}

fn rip_reset_windows(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::ResetWindows))
}

fn rip_erase_window(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::EraseWindow))
}

fn rip_erase_view(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::EraseView))
}

fn rip_gotoxy(input: &str) -> IResult<&str, Command> {
    let (rest, (x, y)) = tuple((meganum2, meganum2))(input)?;
    Ok((rest, Command::Gotoxy(XY { x, y })))
}

fn rip_home(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::Home))
}

fn rip_erase_eol(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::EraseEol))
}

fn rip_color(input: &str) -> IResult<&str, Command> {
    let (rest, color) = palette_color(input)?;
    Ok((rest, Command::Color { color }))
}

fn rip_set_palette(input: &str) -> IResult<&str, Command> {
    let mut c = [EGAColor::new(); 16];
    let (rest, ()) = fill(ega_color, &mut c)(input)?;
    Ok((rest, Command::SetPalette { c }))
}

fn rip_one_palette(input: &str) -> IResult<&str, Command> {
    let (rest, (color, value)) = tuple((palette_color, ega_color))(input)?;
    Ok((rest, Command::OnePalette { color, value }))
}

fn rip_write_mode(input: &str) -> IResult<&str, Command> {
    let (rest, mode) = write_mode(input)?;
    Ok((rest, Command::WriteMode { mode }))
}
