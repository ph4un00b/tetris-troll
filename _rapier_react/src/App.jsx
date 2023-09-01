import { Box, Sphere } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import {
  BallCollider,
  CuboidCollider,
  Physics,
  RigidBody,
} from "@react-three/rapier";
import { Suspense } from "react";

export default function App() {
  return (
    <Canvas>
      <Suspense>
        <Physics debug={!false}>
          <SensorScene />
          {/* <BasicScene /> */}
        </Physics>
      </Suspense>
    </Canvas>
  );
}

function SensorScene() {
  return (
    <>
      <group position={[2, 5, 0]} rotation={[0, 0.3, 2]}>
        <RigidBody>
          <Box />
          <CuboidCollider
            args={[1.5, 0.5, 0.5]}
            //? el sensor no genera collision!
            sensor
            onIntersectionEnter={() => console.log("le tocó ( •_•)>⌐■-■!")}
          />
        </RigidBody>
      </group>

      <BallCollider args={[0.5]} />
      <CuboidCollider
        position={[0, -2, 0]}
        args={[5, 0.5, 2]}
      />
    </>
  );
}

function BasicScene() {
  return (
    <>
      <RigidBody colliders={"ball"} restitution={1.5} position={[2, 10, 0]}>
        <Sphere />
        <BallCollider args={[0.5]} position={[-2, 0, 0]} />
        <BallCollider args={[0.5]} position={[2, 0, 0]} />
      </RigidBody>

      <RigidBody
        position={[0, 6, 0]}
        colliders={"cuboid"}
        restitution={1.5}
      >
        <Box />
      </RigidBody>

      <BallCollider args={[0.5]} />
      <CuboidCollider position={[0, -2, 0]} args={[10, 0.5, 1]} />
    </>
  );
}
