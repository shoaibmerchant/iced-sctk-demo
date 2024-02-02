use iced_runtime::keyboard::KeyCode;
use smithay_client_toolkit::seat::keyboard::Keysym;

pub fn keysym_to_keycode(keysym: Keysym) -> Option<KeyCode> {
    match keysym {
        Keysym::_0 => Some(KeyCode::Key0),
        Keysym::_1 => Some(KeyCode::Key1),
        Keysym::_2 => Some(KeyCode::Key2),
        Keysym::_3 => Some(KeyCode::Key3),
        Keysym::_4 => Some(KeyCode::Key4),
        Keysym::_5 => Some(KeyCode::Key5),
        Keysym::_6 => Some(KeyCode::Key6),
        Keysym::_7 => Some(KeyCode::Key7),
        Keysym::_8 => Some(KeyCode::Key8),
        Keysym::_9 => Some(KeyCode::Key9),
        Keysym::a => Some(KeyCode::A),
        Keysym::b => Some(KeyCode::B),
        Keysym::c => Some(KeyCode::C),
        Keysym::d => Some(KeyCode::D),
        Keysym::e => Some(KeyCode::E),
        Keysym::f => Some(KeyCode::F),
        Keysym::g => Some(KeyCode::G),
        Keysym::h => Some(KeyCode::H),
        Keysym::i => Some(KeyCode::I),
        Keysym::j => Some(KeyCode::J),
        Keysym::k => Some(KeyCode::K),
        Keysym::l => Some(KeyCode::L),
        Keysym::m => Some(KeyCode::M),
        Keysym::n => Some(KeyCode::N),
        Keysym::o => Some(KeyCode::O),
        Keysym::p => Some(KeyCode::P),
        Keysym::q => Some(KeyCode::Q),
        Keysym::r => Some(KeyCode::R),
        Keysym::s => Some(KeyCode::S),
        Keysym::t => Some(KeyCode::T),
        Keysym::u => Some(KeyCode::U),
        Keysym::v => Some(KeyCode::V),
        Keysym::w => Some(KeyCode::W),
        Keysym::x => Some(KeyCode::X),
        Keysym::y => Some(KeyCode::Y),
        Keysym::z => Some(KeyCode::Z),
        Keysym::Escape => Some(KeyCode::Escape),
        Keysym::F1 => Some(KeyCode::F1),
        Keysym::F2 => Some(KeyCode::F2),
        Keysym::F3 => Some(KeyCode::F3),
        Keysym::F4 => Some(KeyCode::F4),
        Keysym::F5 => Some(KeyCode::F5),
        Keysym::F6 => Some(KeyCode::F6),
        Keysym::F7 => Some(KeyCode::F7),
        Keysym::F8 => Some(KeyCode::F8),
        Keysym::F9 => Some(KeyCode::F9),
        Keysym::F10 => Some(KeyCode::F10),
        Keysym::F11 => Some(KeyCode::F11),
        Keysym::F12 => Some(KeyCode::F12),
        Keysym::F13 => Some(KeyCode::F13),
        Keysym::F14 => Some(KeyCode::F14),
        Keysym::F15 => Some(KeyCode::F15),
        Keysym::F16 => Some(KeyCode::F16),
        Keysym::F17 => Some(KeyCode::F17),
        Keysym::F18 => Some(KeyCode::F18),
        Keysym::F19 => Some(KeyCode::F19),
        Keysym::F20 => Some(KeyCode::F20),
        Keysym::F21 => Some(KeyCode::F21),
        Keysym::F22 => Some(KeyCode::F22),
        Keysym::F23 => Some(KeyCode::F23),
        Keysym::F24 => Some(KeyCode::F24),
        Keysym::Pause => Some(KeyCode::Pause),
        Keysym::Insert => Some(KeyCode::Insert),
        Keysym::Home => Some(KeyCode::Home),
        Keysym::Delete => Some(KeyCode::Delete),
        Keysym::End => Some(KeyCode::End),
        Keysym::Page_Down => Some(KeyCode::PageDown),
        Keysym::Page_Up => Some(KeyCode::PageUp),
        Keysym::Left => Some(KeyCode::Left),
        Keysym::Up => Some(KeyCode::Up),
        Keysym::Right => Some(KeyCode::Right),
        Keysym::Down => Some(KeyCode::Down),
        Keysym::BackSpace => Some(KeyCode::Backspace),
        Keysym::Return => Some(KeyCode::Enter),
        Keysym::space => Some(KeyCode::Space),
        // Keysym::Compose => Some(KeyCode::Compose),
        Keysym::caret => Some(KeyCode::Caret),
        Keysym::Num_Lock => Some(KeyCode::Numlock),
        Keysym::KP_0 => Some(KeyCode::Numpad0),
        Keysym::KP_1 => Some(KeyCode::Numpad1),
        Keysym::KP_2 => Some(KeyCode::Numpad2),
        Keysym::KP_3 => Some(KeyCode::Numpad3),
        Keysym::KP_4 => Some(KeyCode::Numpad4),
        Keysym::KP_5 => Some(KeyCode::Numpad5),
        Keysym::KP_6 => Some(KeyCode::Numpad6),
        Keysym::KP_7 => Some(KeyCode::Numpad7),
        Keysym::KP_8 => Some(KeyCode::Numpad8),
        Keysym::KP_9 => Some(KeyCode::Numpad9),
        Keysym::KP_Add => Some(KeyCode::NumpadAdd),
        Keysym::KP_Divide => Some(KeyCode::NumpadDivide),
        Keysym::KP_Decimal => Some(KeyCode::NumpadDecimal),
        // Keysym::KP_Comma => Some(KeyCode::NumpadComma),
        Keysym::KP_Enter => Some(KeyCode::NumpadEnter),
        Keysym::KP_Equal => Some(KeyCode::NumpadEquals),
        Keysym::KP_Multiply => Some(KeyCode::NumpadMultiply),
        Keysym::KP_Subtract => Some(KeyCode::NumpadSubtract),
        // Keysym::AbntC1 => Some(KeyCode::AbntC1),
        // Keysym::AbntC2 => Some(KeyCode::AbntC2),
        Keysym::apostrophe => Some(KeyCode::Apostrophe),
        // Keysym::Apps => Some(KeyCode::Apps),
        Keysym::asterisk => Some(KeyCode::Asterisk),
        // Keysym::At => Some(KeyCode::At),
        // Keysym::Ax => Some(KeyCode::Ax),
        Keysym::backslash => Some(KeyCode::Backslash),
        Keysym::XF86_Calculator => Some(KeyCode::Calculator),
        // Keysym::Capital => Some(KeyCode::Capital),
        Keysym::colon => Some(KeyCode::Colon),
        Keysym::comma => Some(KeyCode::Comma),
        // Keysym::Convert => Some(KeyCode::Convert),
        Keysym::equal => Some(KeyCode::Equals),
        Keysym::grave => Some(KeyCode::Grave),
        // Keysym::Kana => Some(KeyCode::Kana),
        Keysym::Kanji => Some(KeyCode::Kanji),
        Keysym::Alt_L => Some(KeyCode::LAlt),
        Keysym::bracketleft => Some(KeyCode::LBracket),
        Keysym::Control_L => Some(KeyCode::LControl),
        Keysym::Shift_L => Some(KeyCode::LShift),
        Keysym::Super_L => Some(KeyCode::LWin),
        Keysym::XF86_Mail => Some(KeyCode::Mail),
        // Keysym::MediaSelect => Some(KeyCode::MediaSelect),
        // Keysym::MediaStop => Some(KeyCode::MediaStop),
        Keysym::minus => Some(KeyCode::Minus),
        Keysym::XF86_AudioMute => Some(KeyCode::Mute),
        Keysym::XF86_MyComputer => Some(KeyCode::MyComputer),
        // Keysym::NavigateForward => Some(KeyCode::NavigateForward),
        // Keysym::NavigateBackward => Some(KeyCode::NavigateBackward),
        // Keysym::NextTrack => Some(KeyCode::NextTrack),
        // Keysym::NoConvert => Some(KeyCode::NoConvert),
        // Keysym::OEM102 => Some(KeyCode::OEM102),
        Keysym::period => Some(KeyCode::Period),
        // Keysym::PlayPause => Some(KeyCode::PlayPause),
        Keysym::plus => Some(KeyCode::Plus),
        // Keysym::Power => Some(KeyCode::Power),
        // Keysym::PrevTrack => Some(KeyCode::PrevTrack),
        Keysym::Alt_R => Some(KeyCode::RAlt),
        Keysym::bracketright => Some(KeyCode::RBracket),
        Keysym::Control_R => Some(KeyCode::RControl),
        Keysym::Shift_R => Some(KeyCode::RShift),
        Keysym::Super_R => Some(KeyCode::RWin),
        Keysym::semicolon => Some(KeyCode::Semicolon),
        Keysym::slash => Some(KeyCode::Slash),
        Keysym::XF86_Sleep => Some(KeyCode::Sleep),
        Keysym::XF86_Stop => Some(KeyCode::Stop),
        // Keysym::Sysrq => Some(KeyCode::Sysrq),
        Keysym::Tab => Some(KeyCode::Tab),
        // Keysym::Underline => Some(KeyCode::Underline),
        // Keysym::Unlabeled => Some(KeyCode::Unlabeled),
        Keysym::XF86_AudioLowerVolume => Some(KeyCode::VolumeDown),
        Keysym::XF86_AudioRaiseVolume => Some(KeyCode::VolumeUp),
        // Keysym::Wake => Some(KeyCode::Wake),
        // Keysym::WebBack => Some(KeyCode::WebBack),
        // Keysym::WebFavorites => Some(KeyCode::WebFavorites),
        // Keysym::WebForward => Some(KeyCode::WebForward),
        // Keysym::WebHome => Some(KeyCode::WebHome),
        // Keysym::WebRefresh => Some(KeyCode::WebRefresh),
        // Keysym::WebSearch => Some(KeyCode::WebSearch),
        // Keysym::WebStop => Some(KeyCode::WebStop),
        Keysym::yen => Some(KeyCode::Yen),
        Keysym::XF86_Copy => Some(KeyCode::Copy),
        Keysym::XF86_Paste => Some(KeyCode::Paste),
        Keysym::XF86_Cut => Some(KeyCode::Cut),
        _ => None,
    }
}