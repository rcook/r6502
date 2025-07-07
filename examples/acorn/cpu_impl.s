.import MOS_INIT
.import MOS_INTERRUPT_HANDLER

.segment "NMI"
    .addr MOS_INTERRUPT_HANDLER

.segment "RESET"
    .addr MOS_INIT

.segment "IRQ"
    .addr MOS_INTERRUPT_HANDLER
