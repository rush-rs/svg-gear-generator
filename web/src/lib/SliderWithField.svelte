<script lang="ts">
    import Slider from '@smui/slider'
    import Textfield from '@smui/textfield'

    export let value: number
    export let min: number
    export let max: number
    export let step: number = 1
    export let label: string
</script>

<div>
    <Slider bind:value {min} {max} {step} />
    <Textfield bind:value {label} type="number" input$min={min} input$max={max} input$step={step} />
</div>

<style lang="scss">
    div {
        display: grid;
        grid-template-columns: 4fr 1fr;
    }
</style>
