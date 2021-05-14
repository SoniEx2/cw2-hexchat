// CW2 - Hexchat Plugin for Content Warnings
// Copyright (C) 2021  Soni L.
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate hexchat_plugin;

mod parsing;

mod hexchat_plugin_ext;

use std::cell::Cell;

use hexchat_plugin::Plugin as HexchatPlugin;
use hexchat_plugin::PluginHandle as Ph;
use hexchat_plugin::CommandHookHandle;
use hexchat_plugin::PrintHookHandle;

use hexchat_plugin_ext::PhExt;

#[derive(Default)]
struct Cw2Plugin {
    cmd_cw: Cell<Option<CommandHookHandle>>,
    cmd_cwmsg: Cell<Option<CommandHookHandle>>,
    print_hooks: Cell<Vec<PrintHookHandle>>,
}

const PLUG_NAME: &'static str = "CW2";
const PLUG_VER: &'static str = "1.0.0";
const PLUG_DESC: &'static str = "Adds support for content warnings.";

const CMD_CW: &'static str = "CW";
const CMD_CW_HELP: &'static str = "Sends a content warned msg. \
          Example usage: /cw [thing] message";
const CMD_CW_EPARSE: &'static str = "Error parsing CW. \
          Example usage: /cw [thing] message";
const CMD_CW_ENOARG: &'static str = "This command requires more arguments. \
          Example usage: /cw [thing] message";

const CMD_CWMSG: &'static str = "CWMSG";
const CMD_CWMSG_HELP: &'static str = "Sends a content warned msg to an user. \
          Example usage: /cwmsg user [thing] message";
const CMD_CWMSG_EPARSE: &'static str = "Error parsing CW. \
          Example usage: /cwmsg user [thing] message";
const CMD_CWMSG_ENOARG: &'static str = "This command requires more arguments. \
          Example usage: /cwmsg user [thing] message";

impl HexchatPlugin for Cw2Plugin {
    fn init(&self, ph: &mut Ph, _: Option<&str>) -> bool {
        ph.register(PLUG_NAME, PLUG_VER, PLUG_DESC);

        ph.make_command(CMD_CW, |ph, arg, arg_eol| {
            if arg.len() < 2 {
                ph.print(CMD_CW_ENOARG);
                return hexchat_plugin::EAT_ALL
            }
            if let Some(message) = parsing::try_to_cw2(arg_eol[1]) {
                ph.ensure_valid_context(|ph| {
                    ph.command(&format!("say {}", message));
                });
            } else {
                ph.print(CMD_CW_EPARSE);
            }
            hexchat_plugin::EAT_ALL
        }).set_help(CMD_CW_HELP).build_into(&self.cmd_cw);

        ph.make_command(CMD_CWMSG, |ph, arg, arg_eol| {
            if arg.len() < 3 {
                ph.print(CMD_CWMSG_ENOARG);
                return hexchat_plugin::EAT_ALL
            }
            let user = arg_eol[1];
            if let Some(message) = parsing::try_to_cw2(arg_eol[2]) {
                ph.ensure_valid_context(|ph| {
                    ph.command(&format!("msg {} {}", user, message));
                });
            } else {
                ph.print(CMD_CWMSG_EPARSE);
            }
            hexchat_plugin::EAT_ALL
        }).set_help(CMD_CWMSG_HELP).build_into(&self.cmd_cwmsg);

        let mut hooks = Vec::new();
        let to_hook = [
            ("Channel Message", 2,),
            ("Channel Msg Hilight", 2,),
            ("Channel Notice", 3,),
            ("Private Message", 2,),
            ("Private Message to Dialog", 2,),
            ("Notice", 2,),
            ("Your Message", 2,),
            ("Notice Send", 2,),
            ("Message Send", 2,),
        ];
        for &(msg, idx) in &to_hook {
            let h = ph.make_print_attrs(msg, move |ph, arg, attrs| {
                if arg.len() < idx {
                    return hexchat_plugin::EAT_NONE
                }
                // hexchat uses 1-indexed arg
                // but hexchat_plugin uses 0-indexed
                // (ah well)
                let x = arg[idx-1];
                if let Some((reason, content)) = parsing::try_parse_cw2(x) {
                    let mut newargs = Vec::new();
                    newargs.extend(arg.iter().map(|x| String::from(*x)));
                    // NOTE: must not start with "[CW ", as that'd cause an
                    // infinite loop!
                    newargs[idx-1] = format!("[Content Hidden: {}]", reason);
                    ph.print(&format!("\x0326*\t[Content Hidden: {}]\x03\x08{}\x08 \x0324(Copy and paste this line to expand)\x03", reason, content));
                    ph.ensure_valid_context(|ph| {
                        let iter = newargs.iter().map(|x| &**x);
                        // TODO ideally we'd avoid recursion but eh it's fine
                        // .-.
                        // (we guess as long as the above isn't "[CW {}]",
                        // this is probably fine.)
                        ph.emit_print_attrs(attrs, msg, iter);
                    });
                    hexchat_plugin::EAT_ALL
                } else {
                    hexchat_plugin::EAT_NONE
                }
            }).set_priority(i32::MAX).build();
            hooks.push(h);
        }
        self.print_hooks.set(hooks);

        ph.print("CW2 plugin loaded!");

        true
    }
}

hexchat_plugin!(Cw2Plugin);
