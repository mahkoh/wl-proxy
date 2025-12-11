#!/usr/bin/python3

import os
from dataclasses import dataclass
from typing import List


@dataclass
class Repo:
    dir: str
    subdirs: List[str]

repos = [
    Repo(
        dir =  'hyprland-protocols',
        subdirs = ['protocols'],
    ),
]

if __name__ == '__main__':
    os.system('git submodule update --init')
    for repo in repos:
        os.system('git fetch')
        os.system('git checkout origin/HEAD')
        print(repo.subdirs)
