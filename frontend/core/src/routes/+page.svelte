<script lang="ts">
	import { createClient } from '@connectrpc/connect';
	import { createGrpcWebTransport } from '@connectrpc/connect-web';

	import { EchoService, SayHelloRequestSchema } from '$gen/proto/smartauto/v1/echo_service_pb';
	import { create } from '@bufbuild/protobuf';

	async function callSayHello() {
		const transport = createGrpcWebTransport({
			baseUrl: 'http://127.0.0.1:3000',
		});
		const client = createClient(EchoService, transport);

		const message = create(SayHelloRequestSchema, {
			name: 'web123',
		});

		console.log('Message: ' + message.name);

		const resp = await client.sayHello(message);

		console.log(resp.message);
	}

	try {
		callSayHello();
	} catch (e) {
		console.log(e);
	}

	import 'http://localhost:5173/src/main.ts';
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<my-element></my-element>
