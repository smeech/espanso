/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::Event;
use super::{ModeProvider, Dispatcher, Executor, KeyInjector, TextInjector};

pub struct DefaultDispatcher<'a> {
  executors: Vec<Box<dyn Executor + 'a>>,
}

impl<'a> DefaultDispatcher<'a> {
  pub fn new(
    event_injector: &'a dyn TextInjector,
    clipboard_injector: &'a dyn TextInjector,
    mode_provider: &'a dyn ModeProvider,
    key_injector: &'a dyn KeyInjector,
  ) -> Self {
    Self {
      executors: vec![
        Box::new(super::executor::text_inject::TextInjectExecutor::new(
          event_injector,
          clipboard_injector,
          mode_provider,
        )),
        Box::new(super::executor::key_inject::KeyInjectExecutor::new(
          key_injector,
        )),
      ],
    }
  }
}

impl<'a> Dispatcher for DefaultDispatcher<'a> {
  fn dispatch(&self, event: Event) {
    for executor in self.executors.iter() {
      if executor.execute(&event) {
        break;
      }
    }
  }
}
