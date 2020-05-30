#!/bin/bash

set -e

tstart=$(date +%s%N)
cstart=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage)

sleep 1 #sample interval

tstop=$(date +%s%N)
cstop=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage)

calc() { awk "BEGIN{printf \"%.2f\", $*}"; }
cpu=$(calc "($cstop - $cstart) / ($tstop - $tstart) * 100")
mem=$(cat /sys/fs/cgroup/memory/memory.usage_in_bytes  | awk '{ byte =$1 /1024/1024; print byte "m" }')
now=$(date +%s)

echo $now $mem $cpu
