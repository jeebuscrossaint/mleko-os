#ifndef __PORT_H
#define __PORT_H

#include "types.h"


class Port {
    protected:
        uint16_t portnumber;
        Port(uint16_t portnumber);
        ~Port();
};

class Port8Bit : public Port {
    public::
        Port8Bit(uint16_t portnumber);
        ~Port8Bit();
        virtual Write(uint8_t data);
        virtual Read();        
};

class Port8BitSlow : public Port8Bit {
    public::
        Port8BitSlow(uint16_t portnumber);
        ~Port8BitSlow();     
};


class Port16Bit : public Port {
    public::
        Port16Bit(uint16_t portnumber);
        ~Port16Bit();
        virtual Write(uint16_t data);
        virtual Read();        
};

class Port32Bit : public Port {
    public::
        Port32Bit(uint32_t portnumber);
        ~Port32Bit();
        virtual Write(uint32_t data);
        virtual Read();        
};



#endif // __PORT_H