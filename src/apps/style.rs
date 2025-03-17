
use super::lib::*;

pub fn global_style() -> StyleSource {
    css!(
        r#"
            body {
                background: linear-gradient(320deg, rgb(238, 142, 190), rgb(233, 235, 127), rgb(80, 235, 196));
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                margin: 0;
                padding: 0;
            }
        "#
    )
}

pub fn input_button() -> Style {
    style!(r#"
        margin: 8px;
        width: 40px;
        font-size: 35px;
        box-sizing: border-box;
        background-color: rgb(255, 255, 255);
        border-radius: 500px;
        border: 0px;
        color: rgb(0, 0, 0);
        box-shadow: 10px 10px 20px 0px rgba(0, 0, 0, 0.2);
        border-bottom: solid 4px rgb(155, 155, 155);
        cursor: pointer;

        &:hover {
            transform: translateY(2px);
            box-shadow: 0px 0px 15px 0px rgba(0, 0, 0, 0.2);
        }

        &:active {
            transform: translateY(4px);
            border-bottom: 0px;
            box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.2);
        }
    "#).unwrap()
}

pub fn display_style() -> Style {
    style!(r#"
        margin-top: 0px;
        margin-bottom: 20px;
        margin-left: 0x;
        margin-right: 0px;
        padding-right: 20px;
        padding-top: 0px;
        padding-bottom: 20px;
        width: 220px;
        height: 45px;
        text-align: right;
        font-size: 45px;
        border: 0px;
        border-radius: 20px;
        background-color: rgb(255, 255, 255);
        border-bottom: solid 3px rgb(155, 155, 155);
        box-shadow: 10px 10px 20px 0px rgba(0, 0, 0, 0.2);
    "#).unwrap()
}

pub fn calculator_style() -> Style{
    style!(r#"
        margin: 0px;
        padding: 0px;
        border-radius: 50px;
        width: 350px;
        height: 500px;
        background: rgb(255, 255, 255, 0.6);
        box-shadow: 10px 10px 100px 0px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    "#).unwrap()
}

pub fn not_line_break_style() -> Style {
    style!(r#"
        white-space: nowrap;
    "#).unwrap()
}
