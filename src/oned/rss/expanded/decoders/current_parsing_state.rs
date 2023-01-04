/*
 * Copyright (C) 2010 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Numeric,
    Alpha,
    IsoIec646,
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
pub struct CurrentParsingState {
    position: usize,
    encoding: State,
}

impl CurrentParsingState {
    pub fn new() -> Self {
        Self {
            position: 0,
            encoding: State::Numeric,
        }
    }

    pub fn getPosition(&self) -> usize {
        self.position
    }

    pub fn setPosition(&mut self, position: usize) {
        self.position = position
    }

    pub fn incrementPosition(&mut self, delta: usize) {
        self.position += delta;
    }

    pub fn isAlpha(&self) -> bool {
        self.encoding == State::Alpha
    }

    pub fn isNumeric(&self) -> bool {
        self.encoding == State::Numeric
    }

    pub fn isIsoIec646(&self) -> bool {
        self.encoding == State::IsoIec646
    }

    pub fn setNumeric(&mut self) {
        self.encoding = State::Numeric;
    }

    pub fn setAlpha(&mut self) {
        self.encoding = State::Alpha;
    }

    pub fn setIsoIec646(&mut self) {
        self.encoding = State::IsoIec646;
    }
}

impl Default for CurrentParsingState {
    fn default() -> Self {
        Self::new()
    }
}
