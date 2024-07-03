use anchor_lang::prelude::*;

declare_id!("EJeQnCNazSMJkKF2B91no1fms1VPHRDKM7tNs7quL3rT");

#[program]
mod programa_contador {
    use super::*;


    pub fn crear_contador(ctx: Context<Crear>, primer_numero: u64) -> Result<()> {
        ctx.accounts.contador.numero = primer_numero;
        ctx.accounts.contador.autoridad = ctx.accounts.autoridad.key();
        msg!("creando un contador con numero inicial {} ", primer_numero);
        Ok(())
    }

    pub fn borrar_contador(_ctx: Context<Borrar>) -> Result<()> {
        msg!("Contador eliminado");
        Ok(())
    }

    pub fn actualizar_contador(ctx: Context<Actualizar>, cantidad: u64) -> Result<()> {
        ctx.accounts.contador.numero = ctx.accounts.contador.numero + cantidad;

        

        // Registra el evento
        msg!("Contador actualizado a {}", ctx.accounts.contador.numero);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Crear<'info> {
    // 8 bytes para discriminador  + (lo que ocupe tu estructura)
    #[account(init, payer = autoridad, space = 8 + 8 + 32)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Borrar<'info> {
    #[account(mut)]
    pub autoridad: Signer<'info>,
    #[account(
        mut,
        constraint = contador.autoridad == contador.key(),
        close = autoridad
    )]
    pub contador: Account<'info, Contador>,
}

#[derive(Accounts)]
pub struct Actualizar<'info> {
    #[account(mut,constraint = contador.autoridad == autoridad.key())]
    pub contador: Account<'info, Contador>,

    #[account(mut)]
    pub autoridad: Signer<'info>,
}

#[account]
pub struct Contador {
    numero: u64,       // 8 bytes
    autoridad: Pubkey, // 32 bytes
}