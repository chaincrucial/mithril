import React, {useEffect, useState} from 'react';
import {Card, CardGroup, ListGroup} from "react-bootstrap";
import {useSelector} from "react-redux";
import RawJsonButton from "../RawJsonButton";
import SignedEntityType from "../SignedEntityType";
import VerifiedBadge from '../VerifiedBadge';

export default function PendingCertificate(props) {
  const [pendingCertificate, setPendingCertificate] = useState({});
  const aggregator = useSelector((state) => state.settings.selectedAggregator);
  const autoUpdate = useSelector((state) => state.settings.autoUpdate);
  const updateInterval = useSelector((state) => state.settings.updateInterval);

  useEffect(() => {
    if (!autoUpdate) {
      return;
    }

    let fetchPendingCertificate = () => {
      fetch(`${aggregator}/certificate-pending`)
        .then(response => response.status === 200 ? response.json() : {})
        .then(data => setPendingCertificate(data))
        .catch(error => {
          setPendingCertificate({});
          console.error("Fetch certificate-pending error:", error);
        });
    };

    // Fetch it once without waiting
    fetchPendingCertificate();

    const interval = setInterval(fetchPendingCertificate, updateInterval);
    return () => clearInterval(interval);
  }, [aggregator, updateInterval, autoUpdate]);

  return (
    <div className={props.className}>
      <h2>
        Pending Certificate
        {Object.entries(pendingCertificate).length !== 0 &&
          <RawJsonButton
            href={`${aggregator}/certificate-pending`}
            variant="outline-light"
            size="sm"/>
        }
      </h2>

      {Object.entries(pendingCertificate).length === 0
        ? <p>No pending certificate available</p>
        :
        <CardGroup>
          <Card>
            <Card.Body>
              <Card.Title>Beacon</Card.Title>
              <ListGroup className="margin-bottom--md" variant="flush">
                <ListGroup.Item>Network: {pendingCertificate.beacon.network}</ListGroup.Item>
                <ListGroup.Item>Epoch: {pendingCertificate.beacon.epoch}</ListGroup.Item>
              </ListGroup>
              <Card.Title>Entity Type</Card.Title>
              <SignedEntityType signedEntityType={pendingCertificate.entity_type}/>
            </Card.Body>
          </Card>
          <Card>
            <Card.Body>
              <Card.Title>Signers</Card.Title>
              {pendingCertificate.signers.length === 0
                ? <div>No Signers registered</div>
                : <>
                  <ListGroup variant="flush">
                    <ListGroup.Item><b>Party id</b></ListGroup.Item>
                    {pendingCertificate.signers.map(signer =>
                      <ListGroup.Item key={signer.party_id}>
                        {signer.party_id}
                        {signer.verification_key_signature &&
                          <div className="float-end">
                            <VerifiedBadge tooltip="Verified Signer"/>
                          </div>
                        }
                      </ListGroup.Item>
                    )}
                  </ListGroup>
                </>
              }
            </Card.Body>
          </Card>
          <Card>
            <Card.Body>
              <Card.Title>Next Signers</Card.Title>
              {pendingCertificate.next_signers.length === 0
                ? <div>No Signers registered for next epoch</div>
                : <>
                  <ListGroup variant="flush">
                    <ListGroup.Item><b>Party id</b></ListGroup.Item>
                    {pendingCertificate.next_signers.map(signer =>
                      <ListGroup.Item key={signer.party_id}>
                        {signer.party_id}
                        {signer.verification_key_signature &&
                          <div className="float-end">
                            <VerifiedBadge tooltip="Verified Signer"/>
                          </div>
                        }
                      </ListGroup.Item>
                    )}
                  </ListGroup>
                </>
              }
            </Card.Body>
          </Card>
        </CardGroup>
      }
    </div>
  );
}
