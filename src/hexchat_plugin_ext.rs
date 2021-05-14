// This file is part of CW2
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

#![allow(dead_code)]

use std::cell::Cell;
use std::panic::RefUnwindSafe;

use hexchat_plugin::CommandHookHandle;
use hexchat_plugin::Eat;
use hexchat_plugin::EventAttrs;
use hexchat_plugin::PluginHandle as Ph;
use hexchat_plugin::PrintHookHandle;
use hexchat_plugin::ServerHookHandle;
use hexchat_plugin::WordEol as Eol;
use hexchat_plugin::Word;

/// Useful extensions to hexchat_plugin::PluginHandle
pub trait PhExt {
    /// Builds a command hook.
    fn make_command<'a, F>(&'a mut self, cmd: &'a str, cb: F)
        -> CommandBuilder<'a, F>
        where F: Fn(&mut Ph, Word, Eol) -> Eat + 'static + RefUnwindSafe;

    /// Builds a server hook with attributes.
    fn make_server_attrs<'a, F>(&'a mut self, cmd: &'a str, cb: F)
        -> ServerAttrsBuilder<'a, F>
        where
        F: Fn(&mut Ph, Word, Eol, EventAttrs) -> Eat + 'static + RefUnwindSafe;

    /// Builds a print hook with attributes.
    fn make_print_attrs<'a, F>(&'a mut self, msg: &'a str, cb: F)
        -> PrintAttrsBuilder<'a, F>
        where
        F: Fn(&mut Ph, Word, EventAttrs) -> Eat + 'static + RefUnwindSafe;
}

/// Helper for building command hooks. Created with `PhExt::make_command`.
pub struct CommandBuilder<'a, F>
    where F: Fn(&mut Ph, Word, Eol) -> Eat + 'static + RefUnwindSafe
{
    ph: &'a mut Ph,
    cmd: &'a str,
    cb: F,
    pri: i32,
    help: Option<&'a str>,
}

/// Helper for building server hooks. Created with `PhExt::make_server_attrs`.
pub struct ServerAttrsBuilder<'a, F>
    where
    F: Fn(&mut Ph, Word, Eol, EventAttrs) -> Eat + 'static + RefUnwindSafe
{
    ph: &'a mut Ph,
    cmd: &'a str,
    cb: F,
    pri: i32,
}

/// Helper for building print hooks. Created with `PhExt::make_print_attrs`.
pub struct PrintAttrsBuilder<'a, F>
    where
    F: Fn(&mut Ph, Word, EventAttrs) -> Eat + 'static + RefUnwindSafe
{
    ph: &'a mut Ph,
    msg: &'a str,
    cb: F,
    pri: i32,
}

impl PhExt for Ph {
    fn make_command<'a, F>(&'a mut self, cmd: &'a str, cb: F)
        -> CommandBuilder<'a, F>
        where F: Fn(&mut Ph, Word, Eol) -> Eat + 'static + RefUnwindSafe
    {
        CommandBuilder {
            ph: self,
            cmd,
            cb,
            pri: 0,
            help: None,
        }
    }

    fn make_server_attrs<'a, F>(&'a mut self, cmd: &'a str, cb: F)
        -> ServerAttrsBuilder<'a, F>
        where
        F: Fn(&mut Ph, Word, Eol, EventAttrs) -> Eat + 'static + RefUnwindSafe
    {
        ServerAttrsBuilder {
            ph: self,
            cmd,
            cb,
            pri: 0,
        }
    }

    fn make_print_attrs<'a, F>(&'a mut self, msg: &'a str, cb: F)
        -> PrintAttrsBuilder<'a, F>
        where
        F: Fn(&mut Ph, Word, EventAttrs) -> Eat + 'static + RefUnwindSafe
    {
        PrintAttrsBuilder {
            ph: self,
            msg,
            cb,
            pri: 0,
        }
    }
}

impl<'a, F> CommandBuilder<'a, F>
    where F: Fn(&mut Ph, Word, Eol) -> Eat + 'static + RefUnwindSafe
{
    /// Sets the priority of this hook. The default is `PRI_NORM` or `0`.
    pub fn set_priority(self, priority: i32) -> Self {
        Self { pri: priority, ..self }
    }

    /// Sets the help message of this hook. The default is `None`.
    pub fn set_help<S: Into<Option<&'a str>>>(self, help: S) -> Self {
        Self { help: help.into(), ..self }
    }

    /// Registers the hook and stores the handle at the given `Cell`.
    pub fn build_into(self, handle: &Cell<Option<CommandHookHandle>>) {
        handle.set(Some(self.build()));
    }

    /// Registers the hook and returns the handle for it.
    pub fn build(self) -> CommandHookHandle {
        self.ph.hook_command(self.cmd, self.cb, self.pri, self.help)
    }
}

impl<'a, F> ServerAttrsBuilder<'a, F>
    where
    F: Fn(&mut Ph, Word, Eol, EventAttrs) -> Eat + 'static + RefUnwindSafe
{
    /// Sets the priority of this hook. The default is `PRI_NORM` or `0`.
    pub fn set_priority(self, priority: i32) -> Self {
        Self { pri: priority, ..self }
    }

    /// Registers the hook and stores the handle at the given `Cell`.
    pub fn build_into(self, handle: &Cell<Option<ServerHookHandle>>) {
        handle.set(Some(self.build()));
    }

    /// Registers the hook and returns the handle for it.
    pub fn build(self) -> ServerHookHandle {
        self.ph.hook_server_attrs(self.cmd, self.cb, self.pri)
    }
}

impl<'a, F> PrintAttrsBuilder<'a, F>
    where
    F: Fn(&mut Ph, Word, EventAttrs) -> Eat + 'static + RefUnwindSafe
{
    /// Sets the priority of this hook. The default is `PRI_NORM` or `0`.
    pub fn set_priority(self, priority: i32) -> Self {
        Self { pri: priority, ..self }
    }

    /// Registers the hook and stores the handle at the given `Cell`.
    pub fn build_into(self, handle: &Cell<Option<PrintHookHandle>>) {
        handle.set(Some(self.build()));
    }

    /// Registers the hook and returns the handle for it.
    pub fn build(self) -> PrintHookHandle {
        self.ph.hook_print_attrs(self.msg, self.cb, self.pri)
    }
}
