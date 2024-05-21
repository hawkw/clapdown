#![doc = include_str!("../README.md")]
use std::{collections::HashMap, fmt};

pub fn to_markdown_string(cmd: &clap::Command) -> String {
    Settings::new().command_to_string(cmd)
}

/// Configures how to produce Markdown for a [`clap::Command`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings<'a> {
    subcommands: Subcommands,
    heading_level: usize,
    prompt: Option<&'a str>,
    monospace_headings: bool,
    highlight_subcommand_name: bool,
}

/// Determines how to format a command's subcommands (if it has any).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Subcommands {
    /// Subcommands will be formatted as subheadings of the main command.
    ///
    /// If a command or subcommand has subcommands of its own, the list of
    /// subcommands will link to the section headings for those subcommands.
    ///
    /// # Examples
    /// A `clap` command with subcommands, like this:
    ///
    /// ```rust
    /// # use clap::{Parser, CommandFactory, Subcommand};
    ///
    /// /// My great clap command
    /// #[derive(Debug, Parser)]
    /// pub struct MyCommand {
    ///     #[clap(subcommand)]
    ///     subcommand: MySubcommand,
    /// }
    ///
    /// #[derive(Debug, Subcommand)]
    /// pub enum MySubcommand {
    ///     /// A subcommandpi
    ///     DoThings,
    ///     /// Another subcommand
    ///     DoOtherThings,
    /// }
    ///
    /// let formatted = clapdown::Formatter::new()
    ///     .subcommand_mode(clapdown::Subcommands::Flatten)
    ///     .command_to_string(&MyCommand::command());
    /// println!("{formatted}");
    /// ```
    ///
    /// will produce the following output:
    ///
    /// ***
    ///# `$ `**`clapdown`**
    ///
    /// My great clap command
    ///
    ///
    /// ```text
    /// Usage: clapdown do-things
    ///        clapdown do-other-things
    /// ```
    ///
    /// ## subcommands
    ///
    ///  - **[`do-things`](#$--clapdown-do-things)**: A subcommandpi
    ///  - **[`do-other-things`](#$--clapdown-do-other-things)**: Another subcommand
    ///
    /// ## `$ clapdown `**`do-things`**
    ///
    /// A subcommandpi
    ///
    ///
    /// ```text
    /// Usage: do-things
    /// ```
    ///
    ///
    /// ## `$ clapdown `**`do-other-things`**
    ///
    /// Another subcommand
    ///
    ///
    /// ```text
    /// Usage: do-other-things
    /// ```
    ///
    /// ***
    ///
    #[default]
    Flatten,
    /// Subcommands will link to their own markdown files in a subdirectory
    /// named after the main command.
    Linked,
    /// Subcommands will not be formatted. Lists of subcommands will still be
    /// generated, if the command has subcommands.
    None,
}

impl Settings<'static> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            subcommands: Subcommands::Flatten,
            heading_level: 1,
            prompt: Some("$"),
            monospace_headings: true,
            highlight_subcommand_name: true,
        }
    }
}

impl<'a> Settings<'a> {
    #[must_use]
    pub fn with_prompt(self, prompt: impl Into<Option<&'a str>>) -> Self {
        Self {
            prompt: prompt.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_monospace_headings(self, monospace_headings: bool) -> Self {
        Self {
            monospace_headings,
            ..self
        }
    }

    #[must_use]
    pub fn with_subcommand_mode(self, subcommands: Subcommands) -> Self {
        Self {
            subcommands,
            ..self
        }
    }

    #[must_use]
    pub fn with_highlighted_subcommand_names(self, highlight_subcommand_name: bool) -> Self {
        Self {
            highlight_subcommand_name,
            ..self
        }
    }

    pub fn command_to_string(&self, cmd: &clap::Command) -> String {
        let mut s = String::new();
        self.fmt_command(cmd, &mut s)
            .expect("writing to a string should never fail");
        s
    }

    pub fn fmt_command(&self, cmd: &clap::Command, mut writer: impl fmt::Write) -> fmt::Result {
        let mut cmd = cmd.clone().flatten_help(true);
        self.fmt_command_at_level(&mut cmd, &mut writer, self.heading_level, "")
    }

    fn fmt_command_at_level(
        &self,
        cmd: &mut clap::Command,
        writer: &mut impl fmt::Write,
        heading_level: usize,
        parent_name: &str,
    ) -> fmt::Result {
        let heading = "#".repeat(heading_level);
        let mono = if self.monospace_headings { "`" } else { "" };
        let (prompt, prompt_pad) = match self.prompt {
            Some(prompt) => (prompt, " "),
            None => ("", ""),
        };
        let bold = if self.highlight_subcommand_name {
            "**"
        } else {
            ""
        };
        writeln!(
            writer,
            "\n{heading} {mono}{prompt}{prompt_pad}{parent_name}{mono}{bold}{mono}{}{mono}{bold}\n",
            cmd.get_name()
        )?;

        let about = cmd.get_long_about().or_else(|| cmd.get_about());
        if let Some(about) = about {
            writeln!(writer, "{about}\n")?;
        }
        let usage = cmd.render_usage().to_string();
        if !usage.is_empty() {
            writeln!(writer, "\n```text\n{usage}\n```\n",)?;
        }

        let name = cmd.get_name();
        if cmd.has_subcommands() {
            writeln!(writer, "{heading}# subcommands\n")?;
            let parent_link_name = parent_name.replace(' ', "-");
            for cmd in cmd.get_subcommands() {
                let subcmd_name = cmd.get_name();

                if subcmd_name != "help" {
                    match self.subcommands {
                        Subcommands::Flatten => {
                            write!(
                                writer,
                                " - **[`{subcmd_name}`](#{parent_link_name}-{name}-{subcmd_name})**",
                            )?;
                        }
                        Subcommands::Linked => {
                            write!(writer, " - **[`{subcmd_name}`]({name}/{subcmd_name}.md)**",)?;
                        }
                        Subcommands::None => {
                            write!(writer, " - **`{subcmd_name}`**")?;
                        }
                    }
                } else {
                    write!(writer, " - **{subcmd_name}**")?;
                }
                if let Some(about) = cmd.get_about() {
                    write!(writer, ": {about}", about = about)?;
                }
                writer.write_char('\n')?;
            }
        }

        let mut headings = HashMap::new();
        for arg in cmd.get_arguments() {
            let heading = arg.get_help_heading().unwrap_or_else(|| {
                if arg.is_positional() {
                    "arguments"
                } else {
                    "options"
                }
            });
            headings.entry(heading).or_insert_with(Vec::new).push(arg);
        }

        for (heading_name, args) in headings {
            writeln!(writer, "\n{heading}# {heading_name}\n")?;
            for arg in args {
                self.fmt_arg(arg, writer)?;
            }
            writer.write_char('\n')?;
        }

        if cmd.has_subcommands() && self.subcommands == Subcommands::Flatten {
            let parent_name = format!("{parent_name}{name} ");
            for cmd in cmd.get_subcommands_mut() {
                if cmd.get_name() == "help" {
                    continue;
                }
                self.fmt_command_at_level(cmd, writer, heading_level + 1, &parent_name)?;
            }
        }

        Ok(())
    }

    pub fn fmt_arg(&self, arg: &clap::Arg, writer: &mut impl fmt::Write) -> fmt::Result {
        writer.write_str(" - ")?;
        if let Some(short) = arg.get_short() {
            write!(
                writer,
                "`-{short}`{}",
                if arg.get_long().is_some() { ", " } else { "" }
            )?;
        };
        if let Some(long) = arg.get_long() {
            write!(writer, "`--{long}`")?;
        }

        self.fmt_arg_suffix(arg, writer)?;

        let help = arg.get_long_help().or_else(|| arg.get_help());
        if let Some(help) = help {
            let help = help.to_string();
            writeln!(writer, ": {}", help.replace('\n', " "))?;
        } else {
            writer.write_char('\n')?;
        }

        let defaults = arg.get_default_values();
        if !defaults.is_empty() {
            write!(writer, "    - **default:** ")?;
            for val in defaults {
                write!(writer, "`{}` ", val.to_string_lossy())?;
            }
            writer.write_char('\n')?;
        }
        let possible = arg.get_possible_values();
        if !possible.is_empty() {
            writeln!(writer, "    - **possible values:** ")?;
            for val in possible {
                write!(writer, "      - `{}` ", val.get_name())?;
                if let Some(help) = val.get_help() {
                    let help = help.to_string();
                    writeln!(writer, ": {}", help.replace('\n', " "))?;
                } else {
                    writer.write_char('\n')?;
                }
            }
            writer.write_char('\n')?;
        }

        #[cfg(feature = "env")]
        if let Some(env) = arg.get_env() {
            writeln!(writer, "    - **env:** `{}`", env.to_string_lossy())?;
        }

        if let Some(delim) = arg.get_value_delimiter() {
            writeln!(writer, "    - **value delimiter:** `{delim}`")?;
        }

        Ok(())
    }

    fn fmt_arg_suffix(&self, arg: &clap::Arg, writer: &mut impl fmt::Write) -> fmt::Result {
        let num_args = arg.get_num_args().unwrap_or_else(|| 1.into());
        let takes_value = num_args.takes_values();
        let count = matches!(*arg.get_action(), clap::ArgAction::Count);
        if !takes_value && !count {
            return Ok(());
        }

        writer.write_str(" `")?;
        let mut need_closing_bracket = false;
        if takes_value && !arg.is_positional() {
            let is_optional_val = arg
                .get_num_args()
                .map(|n| n.min_values() == 0)
                .unwrap_or(true);
            let start = if arg.is_require_equals_set() {
                if is_optional_val {
                    need_closing_bracket = true;
                    "[="
                } else {
                    "="
                }
            } else if is_optional_val {
                need_closing_bracket = true;
                "["
            } else {
                ""
            };
            writer.write_str(start)?;
        }
        if takes_value || arg.is_positional() {
            self.fmt_arg_val(writer, arg)?;
        } else if count {
            writer.write_str("...")?;
        }
        if need_closing_bracket {
            writer.write_char(']')?;
        }

        writer.write_char('`')?;

        Ok(())
    }

    fn fmt_arg_val(&self, writer: &mut impl fmt::Write, arg: &clap::Arg) -> fmt::Result {
        let num_vals = arg.get_num_args().unwrap_or_else(|| 1.into());

        let mut val_names = match arg.get_value_names() {
            Some(names) => names.iter().map(|s| s.to_string()).collect(),
            None => vec![arg.get_id().as_str().to_ascii_uppercase()],
        };
        if val_names.len() == 1 {
            let min = num_vals.min_values().max(1);
            let val_name = val_names.pop().unwrap();
            val_names = vec![val_name; min];
        }
        if !val_names.is_empty() {
            for (n, val_name) in val_names.iter().enumerate() {
                if n > 0 {
                    writer.write_char(' ')?;
                }
                if arg.is_positional() && (num_vals.min_values() == 0 || !arg.is_required_set()) {
                    write!(writer, "[{val_name}]")?;
                } else {
                    write!(writer, "<{val_name}>")?;
                };
            }

            let mut extra_values = false;
            extra_values |= val_names.len() < num_vals.max_values();
            if arg.is_positional() && matches!(*arg.get_action(), clap::ArgAction::Append) {
                extra_values = true;
            }
            if extra_values {
                writer.write_str("...")?;
            }
        }

        Ok(())
    }
}

impl Default for Settings<'static> {
    fn default() -> Self {
        Self::new()
    }
}
