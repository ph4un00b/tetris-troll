import { Box, Sphere, Torus } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { Physics, RigidBody, CuboidCollider } from "@react-three/rapier";
import { Suspense } from "react";

export default function App() {
  return (
    <Canvas>
      <Suspense>
        <Physics debug={!false}>
          <RigidBody colliders={"cuboid"} restitution={1.5}>
            <Box />
          </RigidBody>

          <CuboidCollider position={[0, -2, 0]} args={[5, 0.5, 1]} />
        </Physics>
      </Suspense>
    </Canvas>
  );
}