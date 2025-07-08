.import MOS_INIT
.import MOS_IRQ_ENTRYPOINT

.segment "NMI"
    .addr MOS_IRQ_ENTRYPOINT

.segment "RESET"
    .addr MOS_INIT

.segment "IRQ"
    .addr MOS_IRQ_ENTRYPOINT
