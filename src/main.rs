use std::{
  env::current_dir,
  fs::File,
  io::{stdin, Write},
};

use console::style;

use regex::Regex;

// ~ fn replace_input(cli_prompt: &str, if_prompt_empty: &str) -> Result<String>

fn main() -> std::io::Result<()> {
  // !? I can't call current_dir() directly, otherwise Rust throws an error.
  // !? I have placed it into this variable to fix this.
  let folder_name = current_dir()?;
  let nfolder = folder_name.to_str().unwrap().split("/");
  let folder_name =
    // ~ Current directory name to lowercase
    String::from(
      nfolder.
      collect::<Vec<&str>>()
      .pop()
      .unwrap()
    )
    .to_ascii_lowercase();
  // ^ nfolder gets moved, so does not need to be dropped directly
  // ^ as Rust does that automatically
  /*
    Ok... so the code above takes the split iterable object, converts it into an &str array.
    then it takes the last folder that it finds from the iterable object to get the current
    directory and then converts it into lowercase to be inline with nodejs standards (all
    names must be lowercase) and I know this is awful practice but I wasn't about to split
    this into several variables and lines
  */
  let import_repo = replace_input("Do you want to import a project (npx degit)? [y/N]", "N");
  // * Guard clause -- executes alternate code when the user wants to import a project
  if import_repo.to_ascii_uppercase() == "Y" {
    let project_name = replace_input("What is your project name?", &folder_name);
    let repo_name = query_user("What is the repository directory?").unwrap();

    let cmd_format = &format!("npx degit {} {}", repo_name, project_name);
    // & Executes using different shell depending on current OS
    if cfg!(target_os = "windows") {
      std::process::Command::new("cmd")
        .args(&["/C", cmd_format])
        .status()
        .expect("Failed to execute npx degit");
    } else {
      std::process::Command::new("sh")
        .args(&["-c", cmd_format])
        .status()
        .expect("Failed to execute npx degit");
    }
    println!("{}", style("Cloned repo without issues! ✅").green());
    return Ok(());
  }

  let project_name_query = format!("What's your project's name? ({:?})", folder_name);

  let project_name = replace_input(&project_name_query, &folder_name);

  let entry_point = replace_input("What is your package's entry point? (index.js)", "index.js");

  let authors = query_user("Who are the authors?")?;

  let mut version: String;
  // * Guard clause -- stops script from continuing if version doesn't meet pattern
  let version_regex = Regex::new("([0-9]+).([0-9]+).([0-9]+)").unwrap();
  loop {
    version = replace_input("What is the version? (1.0.0)", "1.0.0");
    if version_regex
      .captures_iter(&version)
      .collect::<Vec<regex::Captures>>()
      .len()
      != 1
    {
      println!("Must specify a valid version! (<Main>.<Patch>.<Minor>)");
      continue;
    }
    break;
  }

  let description = query_user("What is your package's description?")?;

  let license = replace_input("What is your package's license? (ISC)", "ISC");

  let test = replace_input(
    "Specify a test for your package:",
    "echo \\\"No test specified\\\" && exit 1",
  );
  let keywords = replace_input(
    "Keywords (split by comma, enclose each keyword in \"\")",
    "",
  );

  let mut packages:Vec<String> = vec![];

  let external_imports = replace_input("Does your project use additional packages? [y/N]", "N");

  if &external_imports.to_ascii_uppercase() == "Y" {
    println!(
      "{}",
      style("Type in END to stop adding packages\n").yellow()
    );
    println!("{}", style("Add packages in the format of \"package\":\"version\",\nsimilar to standard JSON structure.\n").yellow());
    loop {
      let package_query = query_user("Add:\n")?;
      // & Break out if user input == END
      if &package_query.to_ascii_uppercase() == "END" {
        break;
      }

      packages.push(format!("   {}", package_query));

      // Put into one value because packages.len() can't be borrowed twice
      let len = packages.len();
      // 1st borrow of 'packages'
      if len > 1 {
        println!("{}", len-2);
        // 2nd borrow of 'packages' (Isn't possible)
        packages[len-2] = format!("{},", packages[len-2]);
        continue
      }
    }
  }

  // & Convert packages into dependencies: { self }
  let packages = format!(
    "
  \"dependencies\": {{
{}
  }}",
    packages.join("\n")
  );

  let mut file_out = File::create("package.json")?;

  let json_out = format!(
    r#"{{
  "name": "{}",
  "main":"{}",
  "description":"{}",
  "scripts":{{
    "test":"{}"
  }},
  "author": "{}",
  "version": "{}",
  "license": "{}",
  "keywords":[{}],{}
}}"#,
    project_name, entry_point, description, test, authors, version, license, keywords, packages
  );

  println!("Data to write to package.json: {}", json_out);
  let confirmation = replace_input("Confirm? [Y/n]", "Y");

  if confirmation.to_ascii_uppercase() != String::from("Y") {
    println!("{}", style("Cancelling operation").red());
    return Ok(());
  }

  file_out.write_all(json_out.as_bytes())?;

  Ok(())
}

fn query_user(query: &str) -> std::io::Result<String> {
  // ~ standout = Standard Out
  let mut standout = String::new();
  print!("{} ", style(&query).blue());
  std::io::stdout().flush()?;
  stdin().read_line(&mut standout)?;
  Ok(String::from(standout.trim()))
}

fn replace_input(input: &str, exit: &str) -> String {
  let user_input = query_user(input).unwrap();
  if String::from(&user_input) == String::from("") {
    return String::from(exit);
  }
  String::from(&user_input)
}
/*
> Example output:
{
  "name": "inknit",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "author": "",
  "license": "ISC"
}
*/
