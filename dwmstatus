#!/usr/bin/env python

import asyncio
import logging
import re
from subprocess import call
from time import strftime


# intervals for each loop
interval_call = 1
interval_net = 1
interval_weather = 3600
interval_cpuinfo = 2
interval_meminfo = 5
interval_timedate = 60

# city for weather information (using wttr.in)
city = "Orebro"

lock = asyncio.Lock()

parse_degrees = re.compile('<span class=".+?">(.+?)</span>')

cpu_last = [0, 0, 0, 0, 0, 0, 0, 0, 0]
cpu_last_sum = 0

net_last_recv = 0
net_last_tran = 0
net_stack_recv = [0.0, 0.0, 0.0]
net_stack_tran = [0.0, 0.0, 0.0]

# MSG variables
net = ""
cpuinfo = ""
meminfo = ""
timedate = ""
weather = ""

async def main():
    await asyncio.gather(
            run_call(),
            get_weather(),
            get_network_speed(),
            get_cpu_info(),
            get_mem_info(),
            get_timedate(),
    )

async def run_call():
    statusmsg = [] 

    while True:
        statusmsg.clear()

        statusmsg.append(net)
        statusmsg.append(cpuinfo)
        statusmsg.append(meminfo)
        statusmsg.append(weather)
        statusmsg.append(timedate)

        final_statusmsg = " ".join(statusmsg)
        logging.debug(["xsetroot", "-name", statusmsg])

        # print(final_statusmsg)
        call(["xsetroot", "-name", final_statusmsg])

        await asyncio.sleep(interval_call)

async def get_weather():
    # wttr.in/:help
    # wttr.in/CITY?T0

    global weather

    interval_error = 5

    while True: 
        code = 'import http.client;' \
        'conn = http.client.HTTPSConnection("www.wttr.in");' \
        'conn.request("GET", "/%s?t0");' \
        'r1 = conn.getresponse().read().decode("utf-8");' \
        'conn.close();' \
        'print(r1);' % city

        proc = await asyncio.create_subprocess_exec(
            'python3', '-c', code, stdout=asyncio.subprocess.PIPE)
        data = await proc.stdout.read()
        lines = data.decode('utf-8').rstrip()
        await proc.wait()

        if "Weather report" not in lines:
            # will try again if there was an error in async subprocess
            weather = ""

            await asyncio.sleep(interval_error)
            interval_error += interval_error

            continue

        wttr = ""

        for num, line in enumerate(lines[lines.find("<pre>"):lines.find("</pre>")].split("\n")):
            if num == 3:
                wttr += line.split(">")[-1].strip() + " "
            elif num == 4:
                degrees = parse_degrees.findall(line)
                break

        if len(degrees) >= 2:
            wttr += "%s°C" % degrees[1]

        if len(degrees) >= 3:
            wttr += "%s%s°C" % (" to " if degrees[1] != None else "", degrees[2])

        logging.debug(wttr)

        async with lock:
            weather = "\ue01d%s" % wttr

        await asyncio.sleep(interval_weather)

async def get_network_speed():
    global net_last_recv
    global net_last_tran
    global net_stack_recv
    global net_stack_tran
    global net

    while True:
        try:
            netread = open("/proc/net/dev", "r").read()
        except Exception as err:
            print(err)
            continue
        
        eno1 = None

        for line in netread.split("\n"):
            if "eno1" in line:
                eno1 = list(map(int, line.split()[1:]))

        if eno1 == None:
            continue

        net_stack_recv.pop(0)
        net_stack_recv.append((eno1[0] - net_last_recv) / 1000000)
        eno1_recv = sum(net_stack_recv) / len(net_stack_recv)

        net_stack_tran.pop(0)
        net_stack_tran.append((eno1[8] - net_last_tran) / 1000000)
        eno1_tran = sum(net_stack_tran) / len(net_stack_tran)

        eno1_recv_tran = "\ue061%0.2f MB/s \ue060%0.2f MB/s" % (eno1_recv, eno1_tran)

        net_last_recv = eno1[0]
        net_last_tran = eno1[8]

        logging.debug(eno1_recv_tran)

        async with lock:
            net = eno1_recv_tran

        await asyncio.sleep(interval_net)

async def get_cpu_info():
    #      user    nice   system  idle      iowait irq   softirq  steal  guest  guest_nice
    # cpu  74608   2520   24433   1117073   6176   4054  0        0      0      0

    # explaination for this shit
    # https://www.idnt.net/en-GB/kb/941772
    global cpu_last
    global cpu_last_sum
    global cpuinfo

    while True:
        try:
            cpuinfo = open("/proc/stat", "r").readline()
        except Exception as err:
            print(err)
            continue

        cpu_now = list(map(int, cpuinfo.split()[1:]))
        cpu_sum = sum(cpu_now)
        cpu_delta = cpu_sum - cpu_last_sum 
        cpu_idle = cpu_now[3] - cpu_last[3]
        cpu_used = cpu_delta - cpu_idle
        cpu_usage = "\ue223%0.2d%%" % int(100 * cpu_used / cpu_delta)

        cpu_last = cpu_now
        cpu_last_sum = cpu_sum

        logging.debug(cpu_usage)

        async with lock:
            cpuinfo = cpu_usage

        await asyncio.sleep(interval_cpuinfo)

async def get_mem_info():
    global meminfo

    while True:
        try:
            memparc = open("/proc/meminfo", "r").read()
        except Exception as err:
            print(err)
            continue

        memtotal = None
        memavailable = None

        for line in memparc.split("\n"):
            if "MemTotal" in line:
                memtotal = int(line.split()[1])
            elif "MemAvailable" in line:
                memavailable = int(line.split()[1])

        if memtotal == None or memavailable == None:
            continue

        memperc = "\ue021%0.2d%%" % int(100 - ((memavailable / memtotal) * 100))

        logging.debug(memperc)

        async with lock:
            meminfo = memperc

        await asyncio.sleep(interval_meminfo)

async def get_timedate():
    global timedate

    while True:
        try:
            timed = "\ue225%s" % strftime("%A %b %Y-%m-%d %H:%M")
        except Exception as err:
            print(err)
            continue

        logging.debug(timed)

        async with lock:
            timedate = timed

        await asyncio.sleep(interval_timedate)

if __name__ == "__main__":
    # logging.basicConfig(level=logging.DEBUG)
    asyncio.run(main())
