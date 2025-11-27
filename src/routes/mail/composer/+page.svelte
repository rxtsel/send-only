<script lang="ts">
  import { Input } from "@/lib/components/ui/input";
  import * as NativeSelect from "$lib/components/ui/native-select";
  import * as Field from "@/lib/components/ui/field";
  import { Textarea } from "@/lib/components/ui/textarea";
  import { InputTag } from "@/lib/components/ui/input-tag";
  import type { Tag } from "@/lib/components/ui/input-tag/input-tag.svelte";
  import { z } from "zod/v4";

  const emailComposerSchema = z.object({
    from: z.string().nonempty("From is required"),
    to: z
      .array(
        z.email({
          message: "Invalid email address",
        }),
      )
      .nonempty("At least one recipient is required"),
    subject: z.string().nonempty("Subject is required"),
    content: z.string().nonempty("Content is required"),
  });

  const fromEmailOptions = [
    "Cristhian <c@rxtsel.dev>",
    "Contact <contact@rxtsel.dev>",
    "Hello <hello@rxtsel.dev>",
  ];

  let toEmails = $state<Tag[]>([]);

  function handleSubmit(event: Event) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const formData = new FormData(form);

    // Get all to emails
    const toEmailsList = formData.getAll("to");

    const data = {
      from: formData.get("from"),
      to: toEmailsList,
      subject: formData.get("subject"),
      content: formData.get("content"),
    };

    const rawData = emailComposerSchema.safeParse(data);

    console.log({ rawData });
  }
</script>

<form class="h-full w-full" onsubmit={handleSubmit} id="compose-email-form">
  <Field.Group class="h-full">
    <Field.Field>
      <Field.Label for="from">From</Field.Label>
      <NativeSelect.Root id="from" name="from">
        {#each fromEmailOptions as option}
          <NativeSelect.Option value={option}>{option}</NativeSelect.Option>
        {/each}
      </NativeSelect.Root>
    </Field.Field>
    <Field.Field>
      <Field.Label for="to">To</Field.Label>
      <InputTag
        id="to"
        type="email"
        bind:value={toEmails}
        name="to"
        placeholder="Add recipient email"
      />
    </Field.Field>
    <Field.Field>
      <Field.Label for="subject">Subject</Field.Label>
      <Input id="subject" type="text" name="subject" placeholder="Subject" />
    </Field.Field>
    <Field.Field class="h-full">
      <Field.Label for="content">Content</Field.Label>
      <Textarea
        id="content"
        name="content"
        placeholder="Enter your email..."
        class="h-full w-full resize-none"
      ></Textarea>
    </Field.Field>
  </Field.Group>
</form>
