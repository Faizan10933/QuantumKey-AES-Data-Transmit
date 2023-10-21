import socket

MCAST_GRP = '239.1.1.1'
MCAST_PORT = 5007
# regarding socket.IP_MULTICAST_TTL
# ---------------------------------
# for all packets sent, after two hops on the network the packet will not 
# be re-sent/broadcast (see https://www.tldp.org/HOWTO/Multicast-HOWTO-6.html)
MULTICAST_TTL = 2

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, MULTICAST_TTL)

# For Python 3, change next line to 'sock.sendto(b"robot", ...' to avoid the
# "bytes-like object is required" msg (https://stackoverflow.com/a/42612820)
# file1 = open("test2.ts", "r")
# with open('data.ts', 'rb') as source_file:
#         source_content = source_file.read()
        # sock.sendto(source_content, (MCAST_GRP, MCAST_PORT))
# with open('data2.ts', 'wb') as destination_file:

    # destination_file.write(source_content)

# res = "robot".encode('utf-8')
# sock.sendto(res, (MCAST_GRP, MCAST_PORT))
# while True:
f = open('test2.ts',mode="rb")
def read1k():
    return f.read(60)
for piece in iter(read1k, ''):
    # process_data(piece)
    print(len(piece))
    print(piece)
    print(type(piece))
    if len(piece)==0:
        break
    sock.sendto(piece, (MCAST_GRP, MCAST_PORT))
