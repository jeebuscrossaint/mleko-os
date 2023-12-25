#include "port.h"

Port::Port(uint16_t portnumber)
{
    this->portnumber = portnumber;
}

Port::~Port()
{
}


Port8Bit::Port8Bit(uint16_t portnumber)
: Port(portnumber)
{
}

Port8Bit::~Port8Bit()
{

} 

void Port8Bit::Write(uint8_t data)
{
    asm volatile("outb %0, %1" : : "a" (data), "Nd" (portnumber));
}

uint8_t Port8Bit::Read()
{
    uint8_t result;
    asm volatile("inb %1, %0 " : "=a" (result) : "Nd" (portnumber));
    return result;
}
//copy slow 8 bit
Port8BitSlow::Port8BitSlow(uint16_t portnumber)
: Port8Bit(portnumber)
{
}

Port8BitSlow::~Port8BitSlow()
{

} 

void Port8BitSlow::Write(uint8_t data)
{
    asm volatile("outb %0, %1\njmp 1f\n1: jmp 1f\n1:" : : "a" (data), "Nd" (portnumber));
}




//copy 16bit

Port16Bit::Port16Bit(uint16_t portnumber)
: Port(portnumber)
{
}

Port16Bit::~Port16Bit()
{

}
void Port16Bit::Write(uint16_t data)
{
    asm volatile("outw %0, %1" : : "a" (data), "Nd" (portnumber));
}

uint16_t Port16Bit::Read()
{
    uint16_t result;
    asm volatile("inw %1, %0 " : "=a" (result) : "Nd" (portnumber));
    return result;
}

//copy 32bit

Port32Bit::Port32Bit(uint32_t portnumber)
: Port(portnumber)
{
}

Port32Bit::~Port32Bit()
{

}

void Port32Bit::Write(uint32_t data)
{
    asm volatile("outl %0, %1" : : "a" (data), "Nd" (portnumber));
}

uint32_t Port32Bit::Read()
{
    uint32_t result;
    asm volatile("inl %1, %0 " : "=a" (result) : "Nd" (portnumber));
    return result;
}