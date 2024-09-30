<script lang="ts">
  import { onMount } from 'svelte'
  import type { Book } from '../types'

  let books: Book[]

  onMount(async () => {
    const res = await fetch(`http://127.0.0.1:3000/get-books`)
    books = await res.json()
  })
</script>

<div class="bg-white py-24 sm:py-32">
  <div class="mx-auto max-w-7xl px-6 lg:px-8">
    <div class="mx-auto max-w-2xl lg:mx-0">
      <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
        Books
      </h2>
      <p class="mt-2 text-lg leading-8 text-gray-600">
        Browse your desired ones.
      </p>
    </div>
    <div
      class="mx-auto mt-10 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 border-t border-gray-200 pt-10 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-3"
    >
      {#if books}
        {#each books as book}
          <article class="flex max-w-xl flex-col items-start justify-between">
            <div class="flex items-center gap-x-4 text-xs">
              <p class="text-gray-500">4.75/5</p>
              <a
                href="/"
                class="relative z-10 rounded-full bg-gray-50 px-3 py-1.5 font-medium text-gray-600 hover:bg-gray-100"
                >{book.genre}</a
              >
            </div>
            <div class="group relative">
              <h3
                class="mt-3 text-lg font-semibold leading-6 text-gray-900 group-hover:text-gray-600"
              >
                <a href="/">
                  <span class="absolute inset-0"></span>
                  {book.title}
                </a>
              </h3>
              <p class="mt-5 line-clamp-3 text-sm leading-6 text-gray-600">
                {book.description}
              </p>
            </div>
            <div class="relative mt-8 flex items-center gap-x-4">
              <span class="h-10 w-10 rounded-full bg-gray-200"></span>
              <div class="text-sm leading-6">
                <p class="font-semibold text-gray-900">
                  <a href="/">
                    <span class="absolute inset-0"></span>
                    {book.author}
                  </a>
                </p>
              </div>
            </div>
          </article>
        {/each}
      {/if}
    </div>
  </div>
</div>
