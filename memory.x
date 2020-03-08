/* Linker script for the NRF52840_QXAA */ 
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K 
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
