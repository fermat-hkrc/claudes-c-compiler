# sizeof(VLA) constant-folds to 1024 bytes in ANY arithmetic — sizeof(v)/sizeof(v[0]) yields 256 for every VLA (the canonical array-length idiom); bare sizeof(v) is correct
> **Filed as [#1144](.../issues/1144)**
