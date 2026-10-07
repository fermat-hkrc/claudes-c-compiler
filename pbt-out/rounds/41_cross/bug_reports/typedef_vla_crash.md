# typedef to a VLA type crashes the compiled binary — `typedef int VA[n]; VA v; v[i]=..;` SEGFAULTs (gcc runs fine); sizeof also wrong (4 vs 20)
> **Filed as [#1147](.../issues/1147)**
