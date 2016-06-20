/************************************************************************
 *                                                                      *
 * Copyright 2016 Urban Hafner                                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/

use board::Board;
use engine::Prior;

use caffe::Net;
use caffe::Phase;
use std::path::Path;

pub struct DCNN {
    net: Net
}

impl DCNN {

    pub fn new() -> DCNN {
        let mut net = Net::new(Path::new("/Users/uh/Personal/detlefs-caffe-dcnn/movepredict.prototxt"), Phase::Test);
        net.copy_trained_layers_from(Path::new("/Users/uh/Personal/detlefs-caffe-dcnn/movepredict.caffemodel"));
        DCNN { net: net }
    }

    pub fn update_priors(&self, priors: &mut Vec<Prior>, board: &Board) {}
}
