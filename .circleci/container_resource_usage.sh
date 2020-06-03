#!/bin/bash

set -e

ncpus=$(nproc)
while true; do
    tstart=$(date +%s%N)                               #nanoseconds since unix epoch timestamp
    cstart=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage) #nanoseconds of cpu usage at tstart

    sleep 5 #sample interval

    tstop=$(date +%s%N)
    cstop=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage) #nanoseoncs of cpu usage at tstop

    calc() { awk "BEGIN{printf \"%.2f\", $*}"; }
    cpu=$(calc "($cstop - $cstart) / (($tstop - $tstart) * $ncpus) * 100")
    mem=$(cat /sys/fs/cgroup/memory/memory.usage_in_bytes  | awk '{ byte=$1 /1024/1024; print byte "m" }')
    now=$(date +%s)

    echo $now $mem $cpu
done >> usage.dat

