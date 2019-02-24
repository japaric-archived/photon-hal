use ll;

pub struct TCPClient {
    socket: ll::sock_handle_t,
}

impl TCPClient {
    pub fn new() -> TCPClient {
        let socket = unsafe {
            ll::socket_create(ll::AF_INET, 1, 6, 1337, 0)
        };
        TCPClient {
            socket: socket,
        }
    }

    pub fn connect(&self, ip: [u8; 4], port: u16) -> Result<(), ()> {

        let mut socket_addr = ll::sockaddr_t {
            sa_family: ll::AF_INET as u16,
            sa_data: [0; 14],
        };

        socket_addr.sa_data[0] = (port >> 8) as u8;
        socket_addr.sa_data[1] = (port & 0xFF) as u8;

        socket_addr.sa_data[2] = ip[0];
        socket_addr.sa_data[3] = ip[1];
        socket_addr.sa_data[4] = ip[2];
        socket_addr.sa_data[5] = ip[3];

        let connect_res = unsafe {
            ll::socket_connect(self.socket, &socket_addr)
        };

        if connect_res == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn send(&self, data: &[u8]) {
        let ptr = data.as_ptr();
        let len = data.len();
        unsafe { ll::socket_send(self.socket, ptr as *const ll::c_void, len); }
    }
}
