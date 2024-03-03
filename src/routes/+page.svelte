<script>
	import { Greet, Query } from '$components';
    import {Button} from "$components/ui/button/index.js";
    import {goto} from "$app/navigation";
</script>

<h1 class="text-center">Welcome to Chess Explorer</h1>
<Button href="/pgn-parser">Go to PGN parser testing</Button>
<br />
<br />
<h2>Simple tests</h2>
<Greet />
<br />
<Query />
