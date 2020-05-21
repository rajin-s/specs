trait Controllable
{
    private
    {
        inputEnabled : bool
    }

    public
    {
        get inputEnabled

        fn self.OnInput [input Vector2] {}

        fn self.EnableInput
        {
            self.inputEnabled = true
        }
        fn self.DisableInput
        {
            self.inputEnabled = false
        }
    }
}

trait Drawable
{
    public
    {
        fn self.Draw {}
    }
}

trait Spatial2D
{
    public
    {
        position : Vector2
        rotation : float
        scale    : Vector2
    }
}

struct Player
{
    is Spatial2D
    
    is Drawable
    {
        fn self.Draw
        {
            (Graphics.Draw
                self.sprite
                self.position
                self.rotation
                self.scale)
        }
    }
    is Controllable
    {
        fn self.OnInput [input Vector2]
        {
            self.velocity = input * self.moveSpeed
        }
    }
    is Updated
    {
        fn self.Update [delta float]
        {
            self.position = self.position + (self.velocity * delta)
        }
    }

    private
    {
        sprite   : Texture
        velocity : Vector2
    }
}