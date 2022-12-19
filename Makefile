prisma-init:
	prisma format && prisma generate && prisma migrate dev --name init
