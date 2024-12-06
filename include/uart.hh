#ifndef UART_HH
#define UART_HH

#include "kernel.hh"

namespace uart {
	// initialize ufart
	void init();

	// send a single character dingaling dingalong dingalong
	void send_char(char c);

	// send a whole string (a pointer to a char array) dingalinag dingaling
	void send_string(const char* str);

	// recieve a character (blocking) who tf hoppin on they X BOXX
	char receive_char();

	// recieve a character (non-blocking) returns -1 if no data
	int receive_char_nonblocking();
} // namespace uart

#endif // UART_HH
