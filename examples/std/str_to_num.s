; Source: http://www.6502.org/source/strings/ascii-to-32bit.html
;
;* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
;*                                                                             *
;*                CONVERT ASCII NUMBER STRING TO 32-BIT BINARY                 *
;*                                                                             *
;*                             by BigDumbDinosaur                              *
;*                                                                             *
;* This 6502 assembly language program converts a null-terminated ASCII number *
;* string into a 32-bit unsigned binary value in little-endian format.  It can *
;* accept a number in binary, octal, decimal or hexadecimal format.            *
;*                                                                             *
;* --------------------------------------------------------------------------- *
;*                                                                             *
;* Copyright (C)1985 by BCS Technology Limited.  All rights reserved.          *
;*                                                                             *
;* Permission is hereby granted to copy and redistribute this software,  prov- *
;* ided this copyright notice remains in the source code & proper  attribution *
;* is given.  Any redistribution, regardless of form, must be at no charge  to *
;* the end user.  This code MAY NOT be incorporated into any package  intended *
;* for sale unless written permission has been given by the copyright holder.  *
;*                                                                             *
;* THERE IS NO WARRANTY OF ANY KIND WITH THIS SOFTWARE.  It's free, so no mat- *
;* ter what, you'll get your money's worth.                                    *
;*                                                                             *
;* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
;
;	Calling Syntax:
;
;		ldx #<numstr
;		ldy #>numstr
;		jsr strbin
;		bcs error
;
;	All registers are modified.  The result of the conversion is left in
;	location PFAC in unsigned, little-endian format (see source code).
;	The contents of PFAC are undefined if strbin exits with an error.
;	The maximum number that can be converted is 4,294,967,295 or (2^32)-1.
;
;	numstr must point to a null-terminated character string in the format:
;
;		[%|@|$]DDD...DDD
;
;	where %, @ or $ are optional radices specifying, respectively, base-2,
;	base-8 or base-16.  If no radix is specified, base-10 is assumed.
;
;	DDD...DDD represents the characters that comprise the number that is
;	to be converted.  Permissible values for each instance of D are:
;
;		Radix  Description  D - D
;		-------------------------
;		  %    Binary       0 - 1
;		  @    Octal        0 - 7
;		 None  Decimal      0 - 9
;		  $    Hexadecimal  0 - 9
;		                    A - F
;		-------------------------
;
;	Conversion is not case-sensitive.  Leading zeros are permissible, but
;	not leading blanks.  The maximum string length including the null
;	terminator is 127.  An error will occur if a character in the string
;	to be converted is not appropriate for the selected radix, the con-
;	verted value exceeds $FFFFFFFF or an undefined radix is specified.
;
;================================================================================
;
;ATOMIC CONSTANTS
;
_origin_ =$02000               ;assembly address
;
;	------------------------------------------
;	Define the above to suit your application.
;	------------------------------------------
;
a_maskuc =%01011111            ;case conversion mask
a_hexnum ='A'-'9'-1            ;hex to decimal difference
n_radix  =4                    ;number of supported radixes
s_fac    =4                    ;binary accumulator size
;
;================================================================================
;
;ZERO PAGE STORAGE
;
.zeropage
ptr01: .word $0000              ; input string pointer
stridx: .byte $00               ; string index
.exportzp pfac
pfac: .dword $00000000          ; primary accumulator
.exportzp sfac
sfac: .dword $00000000          ; secondary accumulator
;
;	------------------------------------------------------
;	Define the above to suit your application.  Moving the
;	accumulators to absolute storage will result in an
;	approximate 20 percent increase in execution time &
;	will require some program restructuring to avoid out-
;	of-range relative branches.
;	------------------------------------------------------
;
;================================================================================
;
;CONVERT NULL-TERMINATED STRING TO 32 BIT BINARY
;
;         *=_origin_
;
.code
.export str_to_num
str_to_num:
         cld
         stx ptr01             ;save string pointer LSB
         sty ptr01+1           ;save string pointer MSB
         lda #0
         ldx #s_fac-1          ;accumulator size
;
strbin01:
         sta pfac,x            ;clear
         dex
         bpl strbin01
;
;	------------------------
;	process radix if present
;	------------------------
;
         tay                   ;starting string index
         clc                   ;assume no error for now
         lda (ptr01),y         ;get a char
         bne strbin02
;
         rts                   ;null string, so exit
;
strbin02:
         ldx #n_radix-1
;
strbin03:
         cmp radxtab,x         ;recognized radix?
         beq strbin04          ;yes
;
         dex
         bpl strbin03          ;try next
;
         stx radxflag          ;assuming decimal...
         inx                   ;which might be wrong
;
strbin04:
         lda basetab,x         ;number bases table
         sta valdnum           ;set valid numeral range
         lda bitstab,x         ;get bits per digit
         sta bitsdig           ;store
         txa                   ;was radix specified?
         beq strbin06          ;no
;
         iny                   ;move past radix
;
strbin05:
         sty stridx            ;save string index
;
;	--------------------------------
;	process number portion of string
;	--------------------------------
;
strbin06:
         clc                   ;assume no error for now
         lda (ptr01),y         ;get numeral
         beq strbin17          ;end of string
;
         inc stridx            ;point to next
         cmp #'a'              ;check char range
         bcc strbin07          ;not ASCII LC
;
         cmp #'z'+1
         bcs strbin08          ;not ASCII LC
;
         and #a_maskuc         ;do case conversion
;
strbin07:
         sec
;
strbin08:
         sbc #'0'              ;change numeral to binary
         bcc strbin16          ;numeral > 0
;
         cmp #10
         bcc strbin09          ;numeral is 0-9
;
         sbc #a_hexnum         ;do a hex adjust
;
strbin09:
         cmp valdnum           ;check range
         bcs strbin17          ;out of range
;
         sta curntnum          ;save processed numeral
         bit radxflag          ;working in base 10?
         bpl strbin11          ;no
;
;	-----------------------------------------------------------
;	Prior to combining the most recent numeral with the partial
;	result, it is necessary to left-shift the partial result
;	result 1 digit.  The operation can be described as N*base,
;	where N is the partial result & base is the number base.
;	N*base with binary, octal & hex is a simple repetitive
;	shift.  A simple shift won't do with decimal, necessitating
;	an (N*8)+(N*2) operation.  PFAC is copied to SFAC to gener-
;	ate the N*2 term.
;	-----------------------------------------------------------
;
         ldx #0
         ldy #s_fac            ;accumulator size
         clc
;
strbin10:
         lda pfac,x            ;N
         rol                   ;N=N*2
         sta sfac,x
         inx
         dey
         bne strbin10
;
         bcs strbin17          ;overflow = error
;
strbin11:
         ldx bitsdig           ;bits per digit
;
strbin12:
         asl pfac              ;compute N*base for binary,...
         rol pfac+1            ;octal &...
         rol pfac+2            ;hex or...
         rol pfac+3            ;N*8 for decimal
         bcs strbin17          ;overflow
;
         dex
         bne strbin12          ;next shift
;
         bit radxflag          ;check base
         bpl strbin14          ;not decimal
;
;	-------------------
;	compute (N*8)+(N*2)
;	-------------------
;
         ldx #0                ;accumulator index
         ldy #s_fac
;
strbin13:
         lda pfac,x            ;N*8
         adc sfac,x            ;N*2
         sta pfac,x            ;now N*10
         inx
         dey
         bne strbin13
;
         bcs strbin17          ;overflow
;
;	-------------------------------------
;	add current numeral to partial result
;	-------------------------------------
;
strbin14:
         clc
         lda pfac              ;N
         adc curntnum          ;N=N+D
         sta pfac
         ldx #1
         ldy #s_fac-1
;
strbin15:
         lda pfac,x
         adc #0                ;account for carry
         sta pfac,x
         inx
         dey
         bne strbin15
;
         bcs strbin17          ;overflow
;
;	----------------------
;	ready for next numeral
;	----------------------
;
         ldy stridx            ;string index
         bpl strbin06          ;get another numeral
;
;	----------------------------------------------
;	if string length > 127 fall through with error
;	----------------------------------------------
;
strbin16:
         sec                   ;flag an error
;
strbin17:
         rts                   ;done
;
;================================================================================
;
;CONVERSION TABLES
;
basetab:  .byte 10,2,8,16       ;number bases per radix
bitstab:  .byte 3,1,3,4         ;bits per digit per radix
radxtab:  .byte " %@$"          ;valid radix symbols
;
;================================================================================
;
;DYNAMIC STORAGE
;
;bitsdig  *=*+1                 ;bits per digit
;curntnum *=*+1                 ;numeral being processed
;radxflag *=*+1                 ;$80 = processing base-10
;valdnum  *=*+1                 ;valid range +1 for selected radix
.data
bitsdig:
    .res 1
curntnum:
    .res 1
radxflag:
    .res 1
valdnum:
    .res 1
;
;================================================================================
	.end