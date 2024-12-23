<script lang="ts">
  import '../app.css';
  let { children } = $props();

  import { createClient } from '@connectrpc/connect';
  import { createGrpcWebTransport } from '@connectrpc/connect-web';

  import { GreeterService, SayHelloRequestSchema } from '$lib/gen/helloworld/v1/helloworld_pb';
  import { create } from '@bufbuild/protobuf';

  const transport = createGrpcWebTransport({
    baseUrl: 'http://127.0.0.1:3000',
  });
  const client = createClient(GreeterService, transport);

  async function callSayHello() {
    var message = create(SayHelloRequestSchema, {
      name: 'web123',
    });

    console.log('Message: ' + message.name);

    const resp = await client.sayHello(message);

    console.log(resp.message);
  }

  callSayHello();
</script>

{@render children()}
