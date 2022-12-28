prisma-init:
	prisma format && prisma generate && prisma migrate dev --name init

nightly:
	rustup override set nightly

stable:
	rustup override set stable
