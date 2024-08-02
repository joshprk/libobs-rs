dump_syms -o target/debug/libobs_segfault_trace.syms target/debug/libobs_segfault_trace.pdb
minidump-stackwalk --symbols-path target/debug/libobs_segfault_trace.syms last_crash.dmp