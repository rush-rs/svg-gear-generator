<script lang="ts">
    import IconButton from '@smui/icon-button'
    import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar'
    import Menu from '@smui/menu'
    import type { MenuComponentDev } from '@smui/menu'
    import List, { Item, Text, Graphic } from '@smui/list'
    import { colorScheme, SchemeKind } from '../stores'
    import './navbar.scss'

    let menu: MenuComponentDev

    function cycleTheme() {
        $colorScheme = ($colorScheme + 1) % (Object.keys(SchemeKind).length / 2)
    }
</script>

<TopAppBar variant="standard">
    <Row>
        <Section>
            <Title>SVG Gear Generator</Title>
        </Section>
        <Section id="toolbar" align="end" toolbar>
            <IconButton
                class="material-icons"
                title={$colorScheme === SchemeKind.Light
                    ? 'Light Theme'
                    : $colorScheme === SchemeKind.System
                    ? 'System Theme'
                    : 'Dark Theme'}
                on:click={cycleTheme}
                >{$colorScheme === SchemeKind.Light
                    ? 'light_mode'
                    : $colorScheme === SchemeKind.System
                    ? 'auto_mode'
                    : 'dark_mode'}</IconButton
            >
        </Section>
        <Section id="menubar" align="end" toolbar>
            <IconButton id="menu-button" class="material-icons" on:click={() => menu.setOpen(true)}
                >more_vert</IconButton
            >
            <Menu bind:this={menu}>
                <List>
                    <Item
                        title={$colorScheme === SchemeKind.Light
                            ? 'Light Theme'
                            : $colorScheme === SchemeKind.System
                            ? 'System Theme'
                            : 'Dark Theme'}
                        on:SMUI:action={cycleTheme}
                    >
                        <Graphic class="material-icons"
                            >{$colorScheme === SchemeKind.Light
                                ? 'light_mode'
                                : $colorScheme === SchemeKind.System
                                ? 'auto_mode'
                                : 'dark_mode'}</Graphic
                        >
                        <Text
                            >{$colorScheme === SchemeKind.Light
                                ? 'Switch to Dark Theme'
                                : $colorScheme === SchemeKind.System
                                ? 'Switch to Light Theme'
                                : 'Switch to System Theme'}</Text
                        >
                    </Item>
                </List>
            </Menu>
        </Section>
    </Row>
</TopAppBar>
