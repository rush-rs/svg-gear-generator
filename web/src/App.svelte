<script lang="ts">
    import { colorScheme, SchemeKind, darkTheme } from './stores'
    import './app.scss'
    import NavBar from './lib/NavBar.svelte'
    import init, { GearSettings, gen_gear_string, Vec2 } from '../wasm-lib-pkg'
    import { onMount } from 'svelte'
    import SliderWithField from './lib/SliderWithField.svelte'

    let scheme = window.localStorage.getItem('color-scheme')
    if (scheme == 'null' || scheme == 'undefined') scheme = 'System'
    $colorScheme = SchemeKind[scheme] !== undefined ? SchemeKind[scheme] : SchemeKind.System
    $: $darkTheme =
        $colorScheme === SchemeKind.Dark ||
        ($colorScheme === SchemeKind.System &&
            window.matchMedia('(prefers-color-scheme: dark)').matches)
    $: window.localStorage.setItem('color-scheme', SchemeKind[$colorScheme])

    let initialized = false
    let svg = ''

    onMount(() => init().then(() => (initialized = true)))

    const settings = {
        ...{
            svgRadius: 90,
            svgInnerRadius: 45,
            svgCenterX: 100,
            svgCenterY: 100,
            svgRotation: 0,
            svgGrooveCount: 10,
            svgGrooveDepth: 0.4,
            svgWidthProportion: 0.2,
            svgCutoff: 4,
            svgPrecision: 3,
        },
        ...JSON.parse(window.localStorage.getItem('settings') || '{}'),
    }
    let {
        svgRadius,
        svgInnerRadius,
        svgCenterX,
        svgCenterY,
        svgRotation,
        svgGrooveCount,
        svgGrooveDepth,
        svgWidthProportion,
        svgCutoff,
        svgPrecision,
    } = settings
    $: if (initialized)
        svg = gen_gear_string(
            new GearSettings(
                svgRadius,
                svgInnerRadius,
                new Vec2(svgCenterX, svgCenterY),
                svgRotation,
                svgGrooveCount,
                svgGrooveDepth,
                svgWidthProportion,
                svgCutoff,
            ),
            'lightblue',
            'black',
            svgPrecision,
        )
    $: window.localStorage.setItem(
        'settings',
        JSON.stringify({
            svgRadius,
            svgInnerRadius,
            svgCenterX,
            svgCenterY,
            svgRotation,
            svgGrooveCount,
            svgGrooveDepth,
            svgWidthProportion,
            svgCutoff,
            svgPrecision,
        }),
    )
</script>

<svelte:head>
    {#if $darkTheme}
        <link rel="stylesheet" href="/theme-dark.css" />
    {/if}
</svelte:head>
<NavBar />

<main>
    <div id="paper" class="mdc-elevation--z4">
        <SliderWithField bind:value={svgRadius} label="Radius" min={0} max={100} />
        <SliderWithField bind:value={svgInnerRadius} label="Inner Radius" min={0} max={100} />
        <SliderWithField bind:value={svgCenterX} label="Center X" min={0} max={200} />
        <SliderWithField bind:value={svgCenterY} label="Center Y" min={0} max={200} />
        <SliderWithField bind:value={svgRotation} label="Rotation" min={0} max={360} />
        <SliderWithField bind:value={svgGrooveCount} label="Groove Count" min={2} max={100} />
        <SliderWithField
            bind:value={svgGrooveDepth}
            label="Groove Depth"
            min={0}
            max={1}
            step={0.05}
        />
        <SliderWithField
            bind:value={svgWidthProportion}
            label="Width Proportion"
            min={-1}
            max={1}
            step={0.1}
        />
        <SliderWithField bind:value={svgCutoff} label="Cutoff After" min={0} max={svgGrooveCount} />
        <svg width="100%" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
            {@html svg}
        </svg>
        <SliderWithField bind:value={svgPrecision} label="Float Precision" min={0} max={10} />
        <code>{svg}</code>
    </div>
</main>

<style lang="scss">
    main {
        padding: 5rem 1rem;
    }

    #paper {
        background-color: var(--clr-height-0-4);
        padding: 1rem 1.5rem;
        border-radius: 0.25rem;
        max-width: 40rem;
        margin: auto;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
</style>
