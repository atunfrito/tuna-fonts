<script>
  let { lleno, tam = 18 } = $props();
</script>

<svg width={tam} height={tam} viewBox="0 0 24 24" fill={lleno ? "currentColor" : "none"} stroke="currentColor" stroke-width="2" stroke-linejoin="round" aria-hidden="true">
  <path d="M12 20s-7-4.3-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.7-7 10-7 10z" />
</svg>
