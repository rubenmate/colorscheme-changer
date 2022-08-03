use std::{process::Command, io::{self, Write}, fs::{File, self}};

fn main() {
    // kitty +kitten themes [options] [theme name to switch to]
    // kitty +kitten themes --reload-in=all Catppuccin-Frappe
    println!("Which colorscheme do you want to change?");
    println!("1- Catppuccin-Frappe\n2- Catppuccin-Latte");
    println!("Enter any number:");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read input");

    let colorscheme = match option.trim() {
        "1" => ("Catppuccin-Frappe", "vim.g.catppuccin_flavour = \"frappe\"\n"),
        "2" => ("Catppuccin-Latte", "vim.g.catppuccin_flavour = \"latte\"\n"),
        _ => panic!("You chose not a valid option")
    };

    let colorscheme_path = "/Users/rubenmate/.config/nvim/lua/user/colorscheme.lua";

    let data = read_file_string(colorscheme_path).unwrap();
    let mut modified_data = String::new();

    for line in data.lines() {
        if line == "vim.g.catppuccin_flavour = \"latte\"" {
            modified_data.push_str(colorscheme.1);
        }
        else if line == "vim.g.catppuccin_flavour = \"frappe\"" {
            modified_data.push_str(colorscheme.1);
        } else {
            modified_data.push_str(line);
            modified_data.push_str("\n")
        }
    }
    modified_data.pop();

    let mut file = File::create(colorscheme_path).expect("create failed");

    file.write_all(modified_data.as_bytes()).expect("write failed");


    let output = Command::new("kitty")
        .args(&["+kitten", "themes","--reload-in=all", colorscheme.0])
        .output()
        .expect("failed to change theme");


    println!("Changed colorscheme!");
    println!("status: {}", output.status);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
