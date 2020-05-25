component struct Transform
{
    private
    {
        position : Vector3
        rotation : Quaternion
        scale    : Vector3
    }

    public
    {
        fn self.GetMatrix -> Matrix4x4 { ... }
    }
}
component struct Renderer
{
    public
    {
        fn self.GetShader -> Shader { ... }
        fn self.GetVertexBuffer -> VertexBuffer { ... }
        fn self.GetRasterizerState -> RasterizerState { ... }
    }
}
component struct Collider
{
    requires
    {
        Transform
    }

    ...
}
component struct RigidBody
{
    requires
    {
        RigidBody
    }

    ...
}

trait Collision
{
    is Abstract
}
component struct StaticBodyCollision
{
    is Collision { ... }
}
component struct RigidBodyCollision
{
    is Collision { ... }
}

struct GraphicsSystems
{
    public
    {
        # Single-element iteration over intersection
        system fn RenderTransform
            entity : (Renderer and Transform)
        {
            # Load data from renderer
            (GL.LoadRendererData
                (entity.Renderer.GetShader)
                (entity.Renderer.GetVertexBuffer)
                (entity.Renderer.GetRasterizerState))

            # Load data from transform
            (GL.LoadWorldTransform
                (entity.Transform.GetMatrix))

            # Submit draw call
            (GL.Draw)
        }

        # Single-element iteration over difference/intersection
        system fn RenderNoTransform
            entity : (Renderer and not Transform)
        {
            # Load data from renderer
            (GL.LoadRendererData
                (entity.Renderer.GetShader)
                (entity.Renderer.GetVertexBuffer)
                (entity.Renderer.GetRasterizerState))

            # Submit draw call
            (GL.Draw)
        }

    }
}

struct GatherCollisionSystems
{
    public
    {
        # Cross-product iteration over a single list
        system fn FindRigidBodyCollisions
            entityA : (RigidBody)
            entityB : (RigidBody)
        {
            let result = (entityA.Collider.Overlaps entityB.Collider)
            if result.hasCollision
            {
                let collisionA = (RigidBodyCollision.New result.normal result.momentum etc ...)
                let collisionB = (RigidBodyCollision.New (result.normal.Reverse) result.momentum etc ...)
                
                (entityA.AddComponent collisionA)
                (entityB.AddComponent collisionB)
            }
        }

        # Cross-product iteration over difference/itersections
        system fn FindStaticBodyCollisions
            entityA : (RigidBody)
            entityB : (Collider and not Rigidbody)
        {
            let result = (entityA.Collider.Overlaps entityB.Collider)
            if result.hasCollision
            {
                let collision = (StaticBodyCollision.New result.normal etc ...)
                (entityA.AddComponent collisionA)
            }
        }
    }
}

struct PhysicsSystems
{
    private
    {
        system fn UpdateTransforms
            entity : (mutable RigidBody)
        {
            (entity.Transform.Translate (Time.Scale entity.RigidBody.velocity))
        }
        system fn ApplyGravity
            entity : (mutable RigidBody)
        {
            (entity.RigidBody.Accelerate (Time.Scale Physics.gravity))
        }
    }
    public
    {
        # Single list iteraction with intersection and super-type
        system fn UpdateColliding
            entity : (mutable RigidBody
                        and mutable Collision)
        {
            (entity.RigidBody.ResolveCollision entity.Collision)
            (entity.RemoveComponent entity.Collision)

            (PhysicsSystems.ApplyGravity entity)
            (PhysicsSystems.UpdateTransforms entity)
        }
        system fn UpdateNonColliding
            entity : (mutable RigidBody
                        and not Collision)
        {
            (PhysicsSystems.ApplyGravity entity)
            (PhysicsSystems.UpdateTransforms entity)
        }
    }
}

component MyComponent
{
    public
    {
    }
}

system Foo
{
    private
    {
        fn Help [t Transform] [c MyComponentA] { ... }
    }

    public
    {
        fn self.UpdateA
            entityA : Entity with { MyComponentA MyComponentB (mutable Transform) }
            entityB : Entity with { Transform (mutable MyComponentA) (mutable MyComponentB) }
        {
            let t = self.entityA.Transform
            let v = (Vector3 10 10 10)

            (t.Translate (Time.Scale v))

            (Foo.Help
                entityA.Transform
                entityB.MyComponentA)
        }

        fn self.UpdateB entity : Entity with { Transform MyComponentA }
        {
            ...
        }

        fn self.UpdateC entity : Entity with MyComponentA
        {
            ...
        }
    }
}

component MyComponent
{
    requires
    {
        mutable Transform
    }

    public
    {
        fn Update [entity (Entity with (mutable MyComponent))]
        {
            let v = (Vector3.New 1 2 3)
            (entity.Transform.Translate (Time.Scale v))
        }
    }
}

component MyOtherComponent
{
    requires
    {
        mutable Transform
        not MyComponent
    }

    public
    {
        system _ fn Update entity : (mutable MyOtherComponent)
        {
            let v = (Vector3.New 3 2 1)
            (entity.Transform.Translate (Time.Scale v))
        }
    }
}