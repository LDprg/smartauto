<script lang="ts">
	import { createClient } from '@connectrpc/connect';
	import { createGrpcWebTransport } from '@connectrpc/connect-web';

	import { GreeterService, SayHelloRequestSchema } from '$gen/helloworld/v1/helloworld_pb';
	import { create } from '@bufbuild/protobuf';

	async function callSayHello() {
		const transport = createGrpcWebTransport({
			baseUrl: 'http://127.0.0.1:3000',
		});
		const client = createClient(GreeterService, transport);

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
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
