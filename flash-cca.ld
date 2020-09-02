__flash_cca__ = FLASH_CCA;

SECTIONS
{
    .flash_cca : AT ( ORIGIN(FLASH) + LENGTH(FLASH) - 44 )
    {
        *(.flash_cca.*);
        . = ALIGN(4);
    } > FLASH
}
