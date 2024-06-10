#ifndef __INTERRUPTS_H__
#define __INTERRUPTS_H__


#include "types.h"
#include "port.h"
#include "gdt.h"


    class InterruptManager
    {
        
        protected:
            struct gateDescriptor
            {
        uint16_t handlerAddressLowBits;
        uint16_t gdt_codeSegmentSelector;
        uint8_t reserved;
        uint8_t access;
        uint16_t handlerAddressHighBits;

    } __attribute__((packed));


    static gateDescriptor interruptDescriptorTable[256];

    struct interruptDescriptorTablePointer {
        uint32_t base;
        uint16_t size;
    } __attribute__((packed));


    static void SetInterruptDescriptorTableEntry(
        uint8_t interruptNumber,
        uint16_t codeSegmentSelectorOffset,
        void (*handler)(),
        uint8_t descriptorPrivilegeLevel,
        uint8_t descriptorType


    );

        public:

            
            InterruptManager(GlobalDescriptorTable* gdt);
            ~InterruptManager();
            
            void activate();
            
            static uint32_t handleInterrupt(uint8_t interruptNumber, uint32_t esp);

            static void ignoreInterruptRequest();
            static void handleInterruptRequest0x01();
            static void handleInterruptRequest0x01();
    }; 

#endif // __INTERRUPTS_H__
