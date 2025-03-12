# Multithreading

Während dem Durchführen von Diagnosen ist es wichtig, dass die Diagnose nebenläufig zum UI durchgeführt wird.
Würde die Diagnose synchron ausgeführt werden, so würde das Rendering und Input-Handling des UI's blockieren, bis die Diagnose vollendet ist.
Daher wird die Diagnose parallel zum UI in einem seperaten Thread ausgeführt.
